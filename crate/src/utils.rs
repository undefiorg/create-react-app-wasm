use cfg_if::cfg_if;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    // https://github.com/rustwasm/console_error_panic_hook
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

cfg_if! {
  // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
  // allocator.
  // https://github.com/rustwasm/wee_alloc
  if #[cfg(feature = "wee_alloc")] {
      extern crate wee_alloc;
      #[global_allocator]
      static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}
