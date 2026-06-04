enum Variant {
    Solid,
    Outline,
}

enum Color {
    Cyan,
    White,
    Gray,
}

fn main2() {
    let variant = Variant::Solid;
    let color = Color::Gray;

    let _ = Variant::Outline;
    let _ = Color::Cyan;
    let _ = Color::White;

    let (v, c): (String, String) = match variant {
        Variant::Solid => (
            "inline-flex justify-center rounded-lg py-2 px-3 text-sm font-semibold transition-colors".into(),
            match color {
                Color::Cyan => "relative overflow-hidden bg-cyan-500 text-white before:absolute before:inset-0 active:before:bg-transparent hover:before:bg-white/10 active:bg-cyan-600 active:text-white/80 before:transition-colors".into(),
                Color::White => "bg-white text-cyan-900 hover:bg-white/90 active:bg-white/90 active:text-cyan-900/70".into(),
                Color::Gray => "bg-gray-800 text-white hover:bg-gray-900 active:bg-gray-800 active:text-white/80".into(),
            },
        ),
        Variant::Outline => (
            "inline-flex justify-center rounded-lg border py-[calc(--spacing(2)-1px)] px-[calc(--spacing(3)-1px)] text-sm transition-colors".into(),
            match color {
                Color::Gray => "border-gray-300 text-gray-700 hover:border-gray-400 active:bg-gray-100 active:text-gray-700/80".into(),
                _ => "".into()
            },
        ),
    };

    let _ = format!("{}{}", v, c);
}


fn main() {

}
