use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub struct Instructions {
    pub(crate) nums_of_philosophers: u64,
    pub(crate) time_to_die: Duration,
    pub(crate) time_to_eat: Duration,
    pub(crate) time_to_sleep: Duration,
    pub(crate) must_eat: Option<u64>,
}

impl Instructions {
    pub(crate) fn new(
        nums_of_philosophers: &str,
        ttd: &str,
        tte: &str,
        tts: &str,
        must_eat: Option<&String>,
    ) -> Result<Self, String> {
        Ok(Instructions {
            nums_of_philosophers: crate::utils::string_to::<u64>(nums_of_philosophers, "error parsing nums of philosophers")?,
            time_to_die: Duration::from_millis(crate::utils::string_to(ttd, "error parsing time to die")?),
            time_to_eat: Duration::from_millis(crate::utils::string_to(tte, "error parsing time to sleep")?),
            time_to_sleep: Duration::from_millis(crate::utils::string_to(tts, "error parsing time to eat")?),
            must_eat: match must_eat {
                Some(value) => match crate::utils::string_to(value, "error parsing must_eat") {
                    Ok(must_eat) => Some(must_eat),
                    Err(err) => return Err(err),
                },
                _ => None,
            },
        })
    }
}
