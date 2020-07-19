// #[cfg(test)]
// mod test {
//     #[test]
//     fn test() {
//         let rw = sdl::sys::SDL_RWFromFile("font.png", "rb");
//         let file_size = sdl2::sys::SDL_RWsize(rw);
//         let mut bytes = Vec::with_capacity(file_size);
//         let read = sdl2::sys::SDL_RWread(rw, &mut bytes as *mut _, 1, file_size);
//         let _ = sdl2::sys::SDL_RWclose(rw); // -1 => closing error
//     }
// }
