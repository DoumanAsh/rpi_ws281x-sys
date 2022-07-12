fn build_source() {
    let is_linux = std::env::var("TARGET").map(|target| target.contains("linux")).unwrap_or(false);

    if !is_linux {
        return;
    }

    let mut cc = cc::Build::new();
    cc.include("src/c")
      .file("src/c/mailbox.c")
      .file("src/c/ws2811.c")
      .file("src/c/pwm.c")
      .file("src/c/pcm.c")
      .file("src/c/dma.c")
      .file("src/c/rpihw.c")
      .shared_flag(false)
      .static_flag(true)
      .warnings(false);

    match cc.try_compile("ws2811") {
        Ok(()) => (),
        Err(error) => panic!("{}", error),
    }
}

fn main() {
    if std::env::var("DOCS_RS").map(|docs| docs == "1").unwrap_or(false) {
        //skip docs.rs build
        return;
    }

    build_source();
}
