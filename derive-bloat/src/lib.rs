#[cfg(feature = "derive")]
mod opt {
    #[derive(Debug)]
    pub enum Opt<T> {
        None,
        Some(T),
    }
}

#[cfg(feature = "manual")]
mod opt {
    use core::fmt;

    pub enum Opt<T> {
        None,
        Some(T),
    }

    impl<T: fmt::Debug> fmt::Debug for Opt<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Opt::None => f.debug_tuple("None").finish(),
                Opt::Some(data) => f.debug_tuple("Some").field(data).finish(),
            }
        }
    }
}

#[cfg(feature = "minimal")]
mod opt {
    use core::fmt;

    pub enum Opt<T> {
        None,
        Some(T),
    }

    impl<T: fmt::Debug> fmt::Debug for Opt<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Opt::None => f.write_str("None"),
                Opt::Some(data) => fmt::Debug::fmt(data, f),
            }
        }
    }
}

#[cfg(feature = "manual-full")]
mod opt {
    use core::fmt;

    pub enum Opt<T> {
        None,
        Some(T),
    }

    impl<T: fmt::Debug> fmt::Debug for Opt<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Opt::None => debug_tuple_new(f, "None").finish(),
                Opt::Some(data) => debug_tuple_new(f, "Some").field(data).finish(),
            }
        }
    }

    pub struct DebugTuple<'a, 'b: 'a> {
        fmt: &'a mut fmt::Formatter<'b>,
        result: fmt::Result,
        fields: usize,
        empty_name: bool,
    }

    impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
        fn is_pretty(&self) -> bool {
            self.fmt.alternate()
        }

        pub fn field(&mut self, value: &dyn fmt::Debug) -> &mut Self {
            self.result = self.result.and_then(|_| {
                if self.is_pretty() {
                    if self.fields == 0 {
                        self.fmt.write_str("(\n")?;
                    }
                    // let mut slot = None;
                    // let mut state = Default::default();
                    // let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
                    value.fmt(&mut self.fmt)?;
                    self.fmt.write_str(",\n")
                } else {
                    let prefix = if self.fields == 0 { "(" } else { ", " };
                    self.fmt.write_str(prefix)?;
                    value.fmt(self.fmt)
                }
            });

            self.fields += 1;
            self
        }

        pub fn finish(&mut self) -> fmt::Result {
            if self.fields > 0 {
                self.result = self.result.and_then(|_| {
                    if self.fields == 1 && self.empty_name && !self.is_pretty() {
                        self.fmt.write_str(",")?;
                    }
                    self.fmt.write_str(")")
                });
            }
            self.result
        }
    }

    pub(super) fn debug_tuple_new<'a, 'b>(
        fmt: &'a mut fmt::Formatter<'b>,
        name: &str,
    ) -> DebugTuple<'a, 'b> {
        let result = fmt.write_str(name);
        DebugTuple {
            fmt,
            result,
            fields: 0,
            empty_name: name.is_empty(),
        }
    }

    // struct PadAdapter<'buf, 'state> {
    //     buf: &'buf mut (dyn fmt::Write + 'buf),
    //     state: &'state mut PadAdapterState,
    // }

    // struct PadAdapterState {
    //     on_newline: bool,
    // }

    // impl Default for PadAdapterState {
    //     fn default() -> Self {
    //         PadAdapterState { on_newline: true }
    //     }
    // }

    // impl<'buf, 'state> PadAdapter<'buf, 'state> {
    //     fn wrap<'slot, 'fmt: 'buf + 'slot>(
    //         fmt: &'fmt mut fmt::Formatter<'_>,
    //         slot: &'slot mut Option<Self>,
    //         state: &'state mut PadAdapterState,
    //     ) -> fmt::Formatter<'slot> {
    //         loop {}
    //     }
    // }

    // impl fmt::Write for PadAdapter<'_, '_> {
    //     fn write_str(&mut self, mut s: &str) -> fmt::Result {
    //         while !s.is_empty() {
    //             if self.state.on_newline {
    //                 self.buf.write_str("    ")?;
    //             }

    //             let split = match s.find('\n') {
    //                 Some(pos) => {
    //                     self.state.on_newline = true;
    //                     pos + 1
    //                 }
    //                 None => {
    //                     self.state.on_newline = false;
    //                     s.len()
    //                 }
    //             };
    //             self.buf.write_str(&s[..split])?;
    //             s = &s[split..];
    //         }

    //         Ok(())
    //     }
    // }
}

#[no_mangle]
pub fn unwrap(r: Result<(), opt::Opt<()>>) {
    r.unwrap()
}
