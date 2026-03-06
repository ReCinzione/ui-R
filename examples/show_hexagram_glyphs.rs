/// Mostra tutti i glifi dei 64 esagrammi in formato HTML
/// 
/// Uso: cargo run --example show_hexagram_glyphs > hexagram_glyphs.html

use prometeo::topology::fractal::bootstrap_fractals;
use prometeo::topology::fractal_visuals::fractal_svg_from_registry;

fn main() {
    let registry = bootstrap_fractals();
    
    println!("<!DOCTYPE html>");
    println!("<html><head>");
    println!("<meta charset='utf-8'>");
    println!("<title>Prometeo - 64 Esagrammi</title>");
    println!("<style>");
    println!("body {{ font-family: monospace; background: #f5f5f5; padding: 20px; }}");
    println!(".grid {{ display: grid; grid-template-columns: repeat(8, 1fr); gap: 20px; }}");
    println!(".glyph {{ background: white; padding: 10px; border: 1px solid #ddd; text-align: center; }}");
    println!(".glyph svg {{ width: 100px; height: 100px; }}");
    println!(".glyph .name {{ font-size: 10px; margin-top: 5px; }}");
    println!(".glyph .id {{ font-size: 8px; color: #999; }}");
    println!(".manual {{ border-color: #4CAF50; border-width: 2px; }}");
    println!(".composed {{ border-color: #2196F3; border-width: 2px; }}");
    println!("</style>");
    println!("</head><body>");
    
    println!("<h1>Prometeo - 64 Esagrammi (I Ching Cognitivo)</h1>");
    println!("<p>Verde = Glifi manuali (0-15) | Blu = Glifi composti (16-63)</p>");
    println!("<div class='grid'>");
    
    for id in 0..64 {
        if let Some(fractal) = registry.get(id) {
            if let Some(svg) = fractal_svg_from_registry(id, &registry) {
                let class = if id < 16 { "glyph manual" } else { "glyph composed" };
                println!("<div class='{}'>{}<div class='name'>{}</div><div class='id'>ID: {}</div></div>", 
                    class, svg, fractal.name, id);
            }
        }
    }
    
    println!("</div>");
    println!("</body></html>");
}
