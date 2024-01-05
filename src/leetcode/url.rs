use std::cell::OnceCell;

static mut URL: OnceCell<Url> = OnceCell::new();

pub struct Url {
    base: String,
    graphql: String,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            base: "https://leetcode.cn".into(),
            graphql: "https://leetcode.cn/graphql".into(),
        }
    }
}

impl Url {
    pub fn global() -> &'static Self {
        unsafe { URL.get_or_init(|| Url::default()) }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn graphql(&self) -> &str {
        &self.graphql
    }
}

mod test {
    use super::Url;

    #[test]
    fn leetcode_url() {
        let base = Url::global().base();
        let graphql = Url::global().graphql();

        assert_eq!(base, "https://leetcode.cn");
        assert_eq!(graphql, "https://leetcode.cn/graphql");
    }
}
