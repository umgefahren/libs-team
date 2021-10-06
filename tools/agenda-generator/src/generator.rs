use chrono::{Duration, NaiveDateTime};
use color_eyre::{
    eyre::{Result, WrapErr},
    Section, SectionExt,
};
use itertools::Itertools;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::de::{DeserializeOwned, Deserializer};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fmt::Write;

#[derive(Default)]
pub struct Generator {
    agenda: String,
    seen: BTreeSet<String>,
}

impl Generator {
    pub fn libs_api_agenda(mut self) -> Result<String> {
        writeln!(
            &mut self.agenda,
            "# Libs-API Meeting {}

###### tags: `Libs Meetings` `Minutes`

**Meeting Link**: https://meet.jit.si/rust-libs-meeting-crxoz2at8hiccp7b3ixf89qgxfymlbwr
**Attendees**: ...

## Agenda

- [Open action items](https://hackmd.io/ovrbJj6CRduRgSA0Wzg2zg)
- Triage
- Anything else?

## Triage
",
            chrono::Utc::now().format("%Y-%m-%d")
        )?;

        self.fcps(String::from("T-libs-api"))?;

        GithubQuery::new("Nominated")
            .labels(&["T-libs-api", "I-nominated"])
            .repo("rust-lang/libs-team")
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Waiting on team")
            .labels(&["T-libs-api", "S-waiting-on-team"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Needs decision")
            .labels(&["T-libs-api", "I-needs-decision"])
            .repo("rust-lang/rust")
            .write(&mut self)?;

        GithubQuery::new("Stalled Tracking Issues")
            .labels(&["T-libs-api", "C-tracking-issue"])
            .repo("rust-lang/rust")
            .sort(Sort::LeastRecentlyUpdated)
            .take(5)
            .write(&mut self)?;

        writeln!(&mut self.agenda,
        "## Actions

- [ ] Reply to all issues/PRs discussed in this meeting, or add them to the [open action items](https://hackmd.io/ovrbJj6CRduRgSA0Wzg2zg).
"
    )?;

        writeln!(&mut self.agenda, "_Generated by [fully-automatic-rust-libs-team-triage-meeting-agenda-generator](https://github.com/rust-lang/libs-team/tree/main/tools/agenda-generator)_")?;
        Ok(self.agenda)
    }

    pub fn libs_agenda(mut self) -> Result<String> {
        writeln!(
            &mut self.agenda,
            "# Libs Meeting {}

###### tags: `Libs Meetings` `Minutes`

**Meeting Link**: https://meet.jit.si/rust-libs-meeting-ujepnbwg2lzqgt6wvrndwimi
**Attendees**: ...

## Agenda

- Triage
    - [Open Action Items](https://hackmd.io/Uehlc0qUQfWfvY1swYWRgQ)
    - Critical Issues
    - MCPs
- Anything else?

## Triage
",
            chrono::Utc::now().format("%Y-%m-%d")
        )?;

        self.fcps(String::from("T-libs"))?;

        GithubQuery::new("Critical")
            .labels(&["T-libs", "P-critical"])
            .labels(&["T-libs-api", "P-critical"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Prioritization Requested")
            .labels(&["T-libs", "I-prioritize"])
            .labels(&["T-libs-api", "I-prioritize"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Nominated")
            .labels(&["T-libs", "I-nominated"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .repo("rust-lang/libs-team")
            .write(&mut self)?;

        GithubQuery::new("Backports")
            .labels(&["T-libs", "stable-nominated"])
            .labels(&["T-libs-api", "stable-nominated"])
            .labels(&["T-libs", "beta-nominated"])
            .labels(&["T-libs-api", "beta-nominated"])
            .exclude_labels(&["beta-accepted"])
            .state(State::Any)
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Regressions")
            .labels(&["T-libs", "regression-untriaged"])
            .labels(&["T-libs-api", "regression-untriaged"])
            .labels(&["T-libs", "regression-from-stable-to-stable"])
            .labels(&["T-libs-api", "regression-from-stable-to-stable"])
            .labels(&["T-libs", "regression-from-stable-to-beta"])
            .labels(&["T-libs-api", "regression-from-stable-to-beta"])
            .labels(&["T-libs", "regression-from-stable-to-nightly"])
            .labels(&["T-libs-api", "regression-from-stable-to-nightly"])
            .exclude_labels(&["T-libs-api", "I-nominated"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        writeln!(&mut self.agenda,
        "## Actions

- [ ] Reply to all issues/PRs discussed in this meeting, or add them to the [open action items](https://hackmd.io/Uehlc0qUQfWfvY1swYWRgQ).
"
    )?;

        writeln!(&mut self.agenda, "_Generated by [fully-automatic-rust-libs-team-triage-meeting-agenda-generator](https://github.com/rust-lang/libs-team/tree/main/tools/agenda-generator)_")?;
        Ok(self.agenda)
    }

    pub fn error_handling_pg_agenda(mut self) -> Result<String> {
        writeln!(
            &mut self.agenda,
            "# Project Group Error Handling Meeting {}

###### tags: `Error Handling` `Minutes`

**Attendees**: ...

## Agenda Items

- [Open action items](https://hackmd.io/@rust-libs/Hyj7kRSld)
- Triage
- Individual Status Updates

## Triage
",
            chrono::Utc::now().format("%Y-%m-%d")
        )?;

        GithubQuery::new("Nominated")
            .labels(&["PG-error-handling", "I-nominated"])
            .repo("rust-lang/rust")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        GithubQuery::new("PG Error Handling")
            .labels(&["PG-error-handling"])
            .repo("rust-lang/rust")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        GithubQuery::new("Area Error Handling")
            .labels(&["A-error-handling"])
            .repo("rust-lang/rust")
            .write(&mut self)?;

        GithubQuery::new("PG Error Handling")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        writeln!(&mut self.agenda,
        "## Actions

- [ ] Reply to all issues/PRs discussed in this meeting, or add them to the [open action items](https://hackmd.io/UrERZvi5RwyxfGvo-RtC6g).
"
    )?;

        writeln!(&mut self.agenda, "_Generated by [fully-automatic-rust-libs-team-triage-meeting-agenda-generator](https://github.com/rust-lang/libs-team/tree/main/tools/agenda-generator)_")?;
        Ok(self.agenda)
    }

    fn fcps(&mut self, label: String) -> Result<()> {
        #[derive(Deserialize, Debug)]
        pub struct FcpWithInfo {
            pub fcp: FcpProposal,
            pub reviews: Vec<(GitHubUser, bool)>,
            pub issue: Issue,
            pub status_comment: IssueComment,
        }

        #[derive(Debug, Deserialize)]
        pub struct FcpProposal {
            pub id: i32,
            pub fk_issue: i32,
            pub fk_initiator: i32,
            pub fk_initiating_comment: i32,
            pub disposition: String,
            pub fk_bot_tracking_comment: i32,
            pub fcp_start: Option<NaiveDateTime>,
            pub fcp_closed: bool,
        }

        #[derive(Deserialize, Debug)]
        pub struct GitHubUser {
            pub id: i32,
            pub login: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Issue {
            pub id: i32,
            pub number: i32,
            pub fk_milestone: Option<i32>,
            pub fk_user: i32,
            pub fk_assignee: Option<i32>,
            pub open: bool,
            pub is_pull_request: bool,
            pub title: String,
            pub body: String,
            pub locked: bool,
            pub closed_at: Option<NaiveDateTime>,
            pub created_at: NaiveDateTime,
            pub updated_at: NaiveDateTime,
            pub labels: Vec<String>,
            pub repository: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct IssueComment {
            pub id: i32,
            pub fk_issue: i32,
            pub fk_user: i32,
            pub body: String,
            pub created_at: NaiveDateTime,
            pub updated_at: NaiveDateTime,
            pub repository: String,
        }

        let mut fcps: Vec<FcpWithInfo> =
            reqwest::blocking::get("https://rfcbot.rs/api/all")?.json()?;
        fcps.retain(|fcp| fcp.issue.labels.contains(&label));

        // Don't filter out FCPs.
        if false {
            let waiting_on_author = "S-waiting-on-author".to_string();
            fcps.retain(|fcp| !fcp.issue.labels.contains(&waiting_on_author));
            let now = chrono::Utc::now().naive_utc();
            fcps.retain(|fcp| {
                let created = fcp.status_comment.created_at;
                let updated = fcp.status_comment.updated_at;
                (now - created) > Duration::weeks(4) && (now - updated) > Duration::days(5)
            });
        }

        let reviewer_count = fcps
            .iter()
            .flat_map(|fcp| fcp.reviews.iter())
            .filter(|review| !review.1)
            .map(|review| &review.0.login)
            .counts();

        let repos = fcps
            .iter()
            .map(|fcp| fcp.issue.repository.as_str())
            .collect::<BTreeSet<_>>();

        writeln!(self.agenda, "### FCPs")?;
        writeln!(self.agenda,)?;
        writeln!(self.agenda, "{} open {} FCPs:", fcps.len(), label)?;

        for repo in repos {
            let fcps = fcps
                .iter()
                .filter(|fcp| fcp.issue.repository == repo)
                .collect::<Vec<_>>();

            writeln!(self.agenda, "<details><summary><a href=\"https://github.com/{}/issues?q=is%3Aopen+label%3AT-libs-api+label%3Aproposed-final-comment-period\">{} <code>{}</code> FCPs</a></summary>\n", repo, fcps.len(), repo)?;

            for fcp in fcps {
                let url = format!(
                    "https://github.com/{}/issues/{}#issuecomment-{}",
                    fcp.issue.repository, fcp.issue.number, fcp.status_comment.id
                );
                write!(
                    self.agenda,
                    "  - [[{} {}]({})] *{}*",
                    fcp.fcp.disposition,
                    fcp.issue.number,
                    url,
                    escape(&fcp.issue.title)
                )?;
                let needed = fcp.reviews.iter().filter(|review| !review.1).count();
                writeln!(self.agenda, " - ({} checkboxes left)", needed)?;

                // TODO I think i need to update the RFCBOT api endpoint to export this info
                // if fcp.concerns {
                //     writeln!(self.agenda, "    Blocked on an open concern.")?;
                // }
            }
            writeln!(self.agenda, "</details>")?;
        }

        writeln!(self.agenda, "<p></p>\n")?;

        for (i, (&reviewer, &num)) in reviewer_count.iter().enumerate() {
            if i != 0 {
                write!(self.agenda, ", ")?;
            }
            write!(
                self.agenda,
                "[{} ({})](https://rfcbot.rs/fcp/{})",
                reviewer, num, reviewer
            )?;
        }
        writeln!(self.agenda,)?;
        writeln!(self.agenda,)?;

        Ok(())
    }

    fn write_issues(&mut self, issues: &[Issue]) -> Result<()> {
        for issue in issues.iter().rev() {
            write!(self.agenda, "  - [[{}]({})]", issue.number, issue.html_url,)?;
            for label in issue.labels.iter().filter(|s| s.starts_with("P-")) {
                write!(self.agenda, " `{}`", label)?;
            }
            writeln!(self.agenda, " *{}*", escape(&issue.title).trim())?;
            if issue
                .labels
                .iter()
                .any(|l| l == "finished-final-comment-period")
            {
                write!(self.agenda, "    FCP finished.")?;
                for label in issue.labels.iter() {
                    if let Some(disposition) = label.strip_prefix("disposition-") {
                        write!(self.agenda, " Should be {}d?", disposition)?;
                    }
                }
                writeln!(self.agenda,)?;
            }
        }

        Ok(())
    }

    fn dedup(&mut self, issues: Vec<Issue>) -> impl Iterator<Item = Issue> + '_ {
        issues
            .into_iter()
            .filter(move |issue| self.seen.insert(issue.html_url.clone()))
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Sort {
    Newest,
    Oldest,
    MostCommented,
    LeastCommented,
    MostRecentlyUpdated,
    LeastRecentlyUpdated,
}

impl Sort {
    fn api_str(&self) -> &'static str {
        match self {
            Sort::Newest => "&sort=created&direction=desc",
            Sort::Oldest => "&sort=created&direction=asc",
            Sort::MostCommented => "&sort=comments&direction=asc",
            Sort::LeastCommented => "&sort=comments&direction=desc",
            Sort::MostRecentlyUpdated => "&sort=updated&direction=desc",
            Sort::LeastRecentlyUpdated => "&sort=updated&direction=asc",
        }
    }

    fn web_ui_str(&self) -> &'static str {
        match self {
            Sort::Newest => "+sort:created-desc",
            Sort::Oldest => "+sort:created-asc",
            Sort::MostCommented => "+sort:comments-desc",
            Sort::LeastCommented => "+sort:comments-asc",
            Sort::MostRecentlyUpdated => "+sort:updated-desc",
            Sort::LeastRecentlyUpdated => "+sort:updated-asc",
        }
    }
}

struct GithubQuery {
    name: &'static str,
    labels: Vec<&'static [&'static str]>,
    excluded_labels: Vec<&'static [&'static str]>,
    repos: Vec<&'static str>,
    sort: Option<Sort>,
    count: Option<usize>,
    state: State,
}

#[allow(dead_code)]
enum State {
    Open,
    Closed,
    Any,
}

impl State {
    fn api_str(&self) -> &'static str {
        match self {
            State::Open => "&state=open",
            State::Closed => "&state=closed",
            State::Any => "&state=all",
        }
    }

    fn web_ui_str(&self) -> &'static str {
        match self {
            State::Open => "+is:open",
            State::Closed => "+is:closed",
            State::Any => "",
        }
    }
}

impl GithubQuery {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            labels: vec![],
            excluded_labels: vec![],
            repos: vec![],
            sort: None,
            count: None,
            state: State::Open,
        }
    }

    fn labels(&mut self, labels: &'static [&'static str]) -> &mut Self {
        self.labels.push(labels);
        self
    }

    fn exclude_labels(&mut self, labels: &'static [&'static str]) -> &mut Self {
        self.excluded_labels.push(labels);
        self
    }

    fn repo(&mut self, repo: &'static str) -> &mut Self {
        self.repos.push(repo);
        self
    }

    fn sort(&mut self, sort: Sort) -> &mut Self {
        self.sort = Some(sort);
        self
    }

    fn take(&mut self, count: usize) -> &mut Self {
        self.count = Some(count);
        self
    }

    fn state(&mut self, state: State) -> &mut Self {
        self.state = state;
        self
    }

    fn write(&mut self, generator: &mut Generator) -> Result<()> {
        writeln!(generator.agenda, "### {}", self.name)?;
        writeln!(generator.agenda,)?;

        let mut empty = true;

        for repo in &self.repos {
            for labels in &self.labels {
                let cs_labels = labels.join(",");
                let mut endpoint = format!("repos/{}/issues?labels={}", repo, cs_labels);

                endpoint += self.state.api_str();

                if let Some(sort) = self.sort {
                    endpoint += sort.api_str();
                }

                let issues = github_api(&endpoint)?;
                let issues = generator.dedup(issues);

                let issues = issues.filter(|issue| {
                    !self.excluded_labels.iter().any(|labels| {
                        labels
                            .iter()
                            .all(|&label| issue.labels.iter().any(|x| x == label))
                    })
                });

                let issues: Vec<_> = if let Some(count) = self.count {
                    issues.take(count).collect()
                } else {
                    issues.collect()
                };

                if issues.is_empty() {
                    continue;
                }

                let url_labels = labels
                    .iter()
                    .map(|label| format!("label:{}", label))
                    .join("+");

                let mut url = format!("https://github.com/{}/issues?q={}", repo, url_labels);

                url += self.state.web_ui_str();

                if let Some(sort) = self.sort {
                    url += sort.web_ui_str();
                }

                writeln!(
                    generator.agenda,
                    "- [{} `{repo}` `{labels}` items]({url})",
                    issues.len(),
                    repo = repo,
                    labels = labels.join("` `"),
                    url = url,
                )?;
                generator.write_issues(&issues)?;

                empty = false;
            }
        }

        if empty {
            writeln!(generator.agenda, "None")?;
        }

        writeln!(generator.agenda)?;

        Ok(())
    }
}

#[derive(Debug)]
struct Fcp<'a> {
    title: &'a str,
    repo: &'a str,
    number: &'a str,
    disposition: &'a str,
    url: &'a str,
    reviewers: Vec<&'a str>,
    concerns: bool,
}

#[derive(Debug, Deserialize)]
struct Issue {
    number: u32,
    html_url: String,
    title: String,
    #[serde(deserialize_with = "deserialize_labels")]
    labels: Vec<String>,
}

fn escape(v: &str) -> String {
    let mut s = String::with_capacity(v.len() + 10);
    v.chars().for_each(|c| {
        match c {
            '_' | '*' | '\\' | '[' | ']' | '-' | '<' | '>' | '`' => s.push('\\'),
            _ => {}
        }
        s.push(c);
    });
    s
}

fn github_api<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
    let url = format!("https://api.github.com/{}", endpoint);
    let mut client = reqwest::blocking::Client::new()
        .get(&url)
        .header(USER_AGENT, "rust-lang libs agenda maker");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        client = client.header(AUTHORIZATION, format!("token {}", token));
    }
    let response = client.send()?;
    let response = response.text()?;
    Ok(serde_json::from_str(&response)
        .wrap_err("response body cannot be deserialized")
        .with_section(|| response.header("Response:"))?)
}

fn deserialize_labels<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    #[derive(Debug, Deserialize)]
    struct Label {
        name: String,
    }
    let v = Vec::<Label>::deserialize(d)?;
    Ok(v.into_iter().map(|l| l.name).collect())
}
