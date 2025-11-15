use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;

use crate::domain::search::{EntriesSearchEngine, MatchedEntry, SearchParameters};
use crate::domain::Entry;

pub struct FuzzySearch {
    matcher: SkimMatcherV2,
}

impl FuzzySearch {
    pub fn new() -> Self {
        FuzzySearch {
            matcher: SkimMatcherV2::default(),
        }
    }
}

impl EntriesSearchEngine for FuzzySearch {
    fn search(&self, entries: &[Entry], search_parameters: &SearchParameters) -> Vec<MatchedEntry> {
        search_impl(&self.matcher, entries, search_parameters)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


fn search_impl<TFuzzyLibImpl: FuzzyMatcher>(
    matcher: &TFuzzyLibImpl,
    entries: &[Entry],
    search_parameters: &SearchParameters,
) -> Vec<MatchedEntry> {
    entries
        .iter()
        .enumerate()
        .map(|(entry_index, entry)| {
            entry
                .as_fields()
                .iter()
                .map(|f| matcher.fuzzy_match(f, &search_parameters.query))
                .flatten()
                .max()
                .map(|max_score| MatchedEntry {
                    entry_index: entry_index,
                    score:       max_score,
                })
        })
        .flatten()
        .collect_vec()
}


#[cfg(test)]
mod tests {
    use fuzzy_matcher::FuzzyMatcher;
    use std::sync::LazyLock;

    use crate::domain::{entry::Entry, search::MatchedEntry};


    mockall::mock! {

        FuzzyMatcherMock {}

        impl FuzzyMatcher for FuzzyMatcherMock {
            fn fuzzy_match(&self, candidate: &str, pattern: &str) -> Option<i64>;
            fn fuzzy_indices(&self, candidate: &str, pattern: &str) -> Option<(i64, Vec<usize>)>;
        }

    }


    const STORAGE_CONTENTS: LazyLock<Vec<Entry>> = std::sync::LazyLock::new(|| {
        vec![
            Entry {
                title:       "The first command".to_string(),
                command:     "Command1".to_string(),
                description: Some("A dummy command 1".to_string()),
            },
            Entry {
                title:       "A statement".to_string(),
                command:     "statement -ls".to_string(),
                description: Some("Completely distinct description".to_string()),
            },
            Entry {
                title:       "The second command".to_string(),
                command:     "command2".to_string(),
                description: Some("A dummy command 2".to_string()),
            },
        ]
    });


    #[test]
    fn test_search_impl_given_executes_fuzzy_match_for_every_field() {
        // Arrange
        let mut seq = mockall::Sequence::new();
        let mut matcher = MockFuzzyMatcherMock::new();
        let mut expected_result = Vec::<MatchedEntry>::new();
        let search_parameters = crate::domain::search::SearchParameters {
            query: "query".to_string(),
        };

        let mut score: i64 = 0;
        for (entry_index, entry_in_storage) in (*STORAGE_CONTENTS).iter().enumerate() {
            for lookup_field in entry_in_storage.as_fields() {
                let this_score = score;
                matcher
                    .expect_fuzzy_match()
                    .with(
                        mockall::predicate::eq(lookup_field.to_string()),
                        mockall::predicate::eq("query"),
                    )
                    .times(1)
                    .in_sequence(&mut seq)
                    .returning(move |_, _| Some(this_score));
                score += 1;
            }
            expected_result.push(MatchedEntry {
                entry_index: entry_index,
                score:       score - 1,
            });
        }

        // Act
        let result = super::search_impl(&matcher, (*STORAGE_CONTENTS).as_ref(), &search_parameters);

        // Assert
        assert_eq!(expected_result, result);
    }
}
