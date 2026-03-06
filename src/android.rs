/// Modulo JNI per Android — entry point del backend embedded.
///
/// Compila con: cargo ndk -t arm64-v8a -o <jniLibs> build --release --features android
///
/// La funzione Java_com_prometeo_app_PrometeoEngine_startServer avvia il server
/// axum su localhost:PORT in un thread OS dedicato con tokio runtime.
/// Il Kotlin si connette via HTTP come se fosse un server remoto.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

use std::sync::OnceLock;

/// Garantisce che il server venga avviato una sola volta.
static SERVER_STARTED: OnceLock<()> = OnceLock::new();

/// JNI entry point.
///
/// Signature Kotlin: `external fun startServer(dataDir: String, port: Int)`
/// Chiamata da: `com.prometeo.app.PrometeoEngine.startServer()`
#[no_mangle]
pub extern "system" fn Java_com_prometeo_app_PrometeoEngine_startServer(
    mut env: JNIEnv,
    _class: JClass,
    data_dir: jni::objects::JString,
    port: jint,
) {
    // Leggi il path della directory dati dall'argomento JVM
    let data_path: String = match env.get_string(&data_dir) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("[android] Errore lettura dataDir: {:?}", e);
            return;
        }
    };

    let port = port as u16;

    SERVER_STARTED.get_or_init(|| {
        // Imposta la cwd sulla directory dati Android
        // (permette ai path relativi del server di funzionare)
        if let Err(e) = std::env::set_current_dir(&data_path) {
            eprintln!("[android] Impossibile impostare cwd={}: {:?}", data_path, e);
        }

        let _ = std::thread::Builder::new()
            .name("prometeo-backend".to_string())
            .spawn(move || {
                // Runtime tokio dedicato per il backend
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("impossibile creare tokio runtime");
                rt.block_on(crate::web::server::run(port));
            });
    });
}
