use serde::Deserialize;

/// GROWI のページのパスを絞り込む設定
#[derive(Debug, PartialEq, Eq)]
pub struct PathFilter {
    /// `/Sandbox/Bootstrap4/` の形式の文字列 (末尾に `/` を付ける)
    parent_path: String,

    /// 多用するので raw の長さを先に測っておく
    len: usize,
}

impl PathFilter {
    /// 指定されたページが self のページに含まれるなら true
    pub fn contains(&self, path: &str) -> bool {
        if path.starts_with(&self.parent_path) {
            return true;
        }

        // parent: "/Sandbox/", path: "/Sandbox" のパターンか確認
        let self_no_suffix = &self.parent_path[..self.len - 1];
        self_no_suffix == path
    }
}

impl From<String> for PathFilter {
    fn from(mut value: String) -> Self {
        if !value.ends_with('/') {
            value.push('/');
        }

        Self {
            len: value.len(),
            parent_path: value,
        }
    }
}

impl<'de> Deserialize<'de> for PathFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_filter(s: &str) -> PathFilter {
        s.to_string().into()
    }

    // tests for PathFilter::contains()
    mod test_contains {
        use super::*;

        use test_case::test_case;

        #[test_case("/Sandbox", "/Sandbox/Bootstrap4" ; "no slash and no slash")]
        #[test_case("/Sandbox", "/Sandbox/Bootstrap4/" ; "no slash and slash")]
        #[test_case("/Sandbox/", "/Sandbox/Bootstrap4" ; "slash and no slash")]
        #[test_case("/Sandbox/", "/Sandbox/Bootstrap4/" ; "slash and slash")]
        fn matches(filter: &str, target: &str) {
            let parent = str_to_filter(filter);

            assert!(parent.contains(target))
        }

        #[test_case("/projects", "/Sandbox/Bootstrap4" ; "no slash and no slash")]
        #[test_case("/projects", "/Sandbox/Bootstrap4/" ; "no slash and slash")]
        #[test_case("/projects/", "/Sandbox/Bootstrap4" ; "slash and no slash")]
        #[test_case("/projects/", "/Sandbox/Bootstrap4/" ; "slash and slash")]
        #[test_case("/Sand", "/Sandbox/Bootstrap4" ; "head maches but not same page")]
        fn not_matches(parent: &str, target: &str) {
            let parent = str_to_filter(parent);

            assert!(!parent.contains(target))
        }

        #[test_case("/Sandbox", "/Sandbox" ; "no slash and no slash")]
        #[test_case("/Sandbox", "/Sandbox/" ; "no slash and slash")]
        #[test_case("/Sandbox/", "/Sandbox" ; "slash and no slash")]
        #[test_case("/Sandbox/", "/Sandbox/" ; "slash and slash")]
        fn same(parent: &str, target: &str) {
            let parent = str_to_filter(parent);

            assert!(parent.contains(target))
        }
    }
}
