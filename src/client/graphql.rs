use serde::Deserialize;

pub const PROBLEM_LIST_QUERY: &str = r#"
query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
  problemsetQuestionList: questionList(
    categorySlug: $categorySlug
    limit: $limit
    skip: $skip
    filters: $filters
  ) {
    total: totalNum
    questions: data {
      questionId
      questionFrontendId
      title
      titleSlug
      difficulty
      isPaidOnly
      acRate
      status
      topicTags { slug }
    }
  }
}
"#;

pub const PROBLEM_DETAIL_QUERY: &str = r#"
query questionData($titleSlug: String!) {
  question(titleSlug: $titleSlug) {
    questionId
    questionFrontendId
    title
    titleSlug
    difficulty
    content
    sampleTestCase
    exampleTestcases
    codeSnippets { lang langSlug code }
  }
}
"#;

pub const DAILY_QUERY: &str = r#"
query questionOfToday {
  activeDailyCodingChallengeQuestion {
    date
    link
    question {
      questionId
      questionFrontendId
      title
      titleSlug
      difficulty
    }
  }
}
"#;

pub const USER_STATUS_QUERY: &str = r#"
query globalData {
  userStatus {
    userId
    username
    isSignedIn
  }
}
"#;

pub const USER_PROFILE_QUERY: &str = r#"
query userProfile($username: String!) {
  allQuestionsCount {
    difficulty
    count
  }
  matchedUser(username: $username) {
    submitStatsGlobal {
      acSubmissionNum {
        difficulty
        count
      }
    }
  }
}
"#;

/// Envelope for a GraphQL response body: `{ "data": { ... } }`.
#[derive(Debug, Deserialize)]
pub struct GqlResponse<T> {
    pub data: Option<T>,
    #[serde(default)]
    pub errors: Option<Vec<GqlError>>,
}

#[derive(Debug, Deserialize)]
pub struct GqlError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ProblemListData {
    #[serde(rename = "problemsetQuestionList")]
    pub list: Option<ProblemList>,
}

#[derive(Debug, Deserialize)]
pub struct ProblemList {
    pub total: i64,
    pub questions: Vec<RawProblemSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawProblemSummary {
    pub question_id: String,
    pub question_frontend_id: String,
    pub title: String,
    pub title_slug: String,
    pub difficulty: String,
    pub is_paid_only: bool,
    pub ac_rate: f64,
    pub status: Option<String>,
    pub topic_tags: Vec<TopicTag>,
}

#[derive(Debug, Deserialize)]
pub struct TopicTag {
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct ProblemDetailData {
    pub question: Option<RawProblemDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawProblemDetail {
    pub question_id: String,
    pub question_frontend_id: String,
    pub title: String,
    pub title_slug: String,
    pub difficulty: String,
    pub content: Option<String>,
    pub sample_test_case: Option<String>,
    pub example_testcases: Option<String>,
    pub code_snippets: Option<Vec<RawCodeSnippet>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCodeSnippet {
    pub lang: String,
    pub lang_slug: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyData {
    #[serde(rename = "activeDailyCodingChallengeQuestion")]
    pub daily: Option<DailyChallenge>,
}

#[derive(Debug, Deserialize)]
pub struct DailyChallenge {
    pub date: String,
    pub link: String,
    pub question: DailyQuestion,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyQuestion {
    pub question_frontend_id: String,
    pub title: String,
    pub title_slug: String,
    pub difficulty: String,
}

#[derive(Debug, Deserialize)]
pub struct UserStatusData {
    #[serde(rename = "userStatus")]
    pub user_status: UserStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatus {
    pub username: Option<String>,
    pub is_signed_in: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileData {
    pub all_questions_count: Vec<DifficultyCount>,
    pub matched_user: Option<MatchedUser>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyCount {
    pub difficulty: String,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchedUser {
    pub submit_stats_global: SubmitStatsGlobal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitStatsGlobal {
    pub ac_submission_num: Vec<DifficultyCount>,
}
