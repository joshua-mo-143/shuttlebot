use serde::Serialize;
use sqlx::{types::Json, PgPool};
use tracing::error;
use serde_json::{value::Value};

#[derive(Clone)]
pub struct DBQueries {
    pub db: PgPool,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Issue {
    #[serde(rename(serialize = "originalPoster"))]
    pub original_poster: Option<String>,
    #[serde(rename(serialize = "discordThreadLink"))]
    pub discord_thread_link: String,
    pub severity: i16,
    #[serde(rename(serialize = "firstResponder"))]
    pub first_responder: Option<String>,
    #[serde(rename(serialize = "githubLink"))]
    pub github_link: Option<String>,
    #[serde(rename(serialize = "resolvedBy"))]
    pub resolved_by: Option<String>,
    pub categories: Option<Vec<String>>,
    #[serde(rename(serialize = "creationDate"))]
    pub creation_date: String,
}

#[derive(Serialize)]
pub struct DashboardData {
    #[serde(rename(serialize = "lastFourWeeksStats"))]
    pub last_four_weeks_stats: Vec<LastFourWeeksStats>,
    #[serde(rename(serialize = "issuesAwaitingResponse"))]
    pub issues_awaiting_response: IssuesAwaitingResponse,
    #[serde(rename(serialize = "issuesOpenedLastWeek"))]
    pub issues_opened_last_week: Vec<IssuesOpenedLastWeek>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct LastFourWeeksStats {
    #[serde(rename(serialize = "dateRange"))]
    pub date_range: String,
    #[serde(rename(serialize = "totalIssues"))]
    pub total_issues: i64,
    #[serde(rename(serialize = "totalElevatedIssues"))]
    pub total_elevated_issues: i64,
    #[serde(rename(serialize = "totalResolvedIssues"))]
    pub total_resolved_issues: i64,
    #[serde(rename(serialize = "totalOneTouchThreads"))]
    pub total_one_touch_threads: i64,
    #[serde(rename(serialize = "extendedThreads"))]
    pub extended_threads: i64,
    #[serde(rename(serialize = "averageResponseTime"))]
    pub average_response_time: Option<String>,
    #[serde(rename(serialize = "bestSolver"))]
    pub best_solver: Option<String>,
    #[serde(rename(serialize = "bestFirstResponder"))]
    pub best_first_responder: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct IssuesOpenedLastWeek {
    pub day: String,
    #[serde(rename(serialize = "totalIssuesPerDay"))]
    pub total_issues_per_day: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct IssuesAwaitingResponse {
    #[serde(rename(serialize = "unansweredThreads"))]
    pub unanswered_threads: i64,
    #[serde(rename(serialize = "unresolvedIssues"))]
    pub unresolved_issues: i64,
    #[serde(rename(serialize = "unresolvedGithubIssues"))]
    pub unresolved_github_issues: i64,
}

impl DBQueries {
    pub async fn discord_get_feedback(
        self, channel_id: String, upvotes: i32, downvotes: i32,
    ) -> Result<(), anyhow::Error> {
         if let Err(e) = sqlx::query("INSERT INTO feedback (discordthreadid, upvotes, downvotes) VALUES ($1, $2, $3) 
                    ON CONFLICT (discordthreadid) DO UPDATE SET upvotes = EXCLUDED.upvotes, downvotes = EXCLUDED.downvotes")
            .bind(channel_id)
            .bind(upvotes)
            .bind(downvotes)
            .execute(&self.db)
            .await {
                error!("Error when getting feedback: {e}");
            }

        Ok(())
        
    }
    
    pub async fn discord_elevate_thread(
        self,
        github_issue_link: String,
        thread_url: String,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET 
                GithubLink = $1, 
                Locked = TRUE,
                LockStatusChangeReason = 'Thread was elevated to GitHub issue'
                WHERE DiscordThreadLink = $2",
        )
        .bind(github_issue_link)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Failed to update SQL record after elevating GitHub issue: {:?}",
                e
            );
        }

        Ok(())
    }

    pub async fn discord_change_locked_status(
        self,
        locked: bool,
        reason: String,
        thread_url: String,
    ) -> Result<(), String> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET
        Locked = $1, 
        LockStatusChangeReason = $2, 
        ResolvedTimedate = CURRENT_TIMESTAMP 
        WHERE DiscordThreadLink = $3",
        )
        .bind(locked)
        .bind(reason)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Error when updating SQL record after thread lock status: {:?}",
                e
            );
        }

        Ok(())
    }

    pub async fn discord_resolve_thread(
        self,
        resolved_by: String,
        thread_url: String,
        message_count: i32,
        usercount: i32,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET
        Locked = true, 
        ResolverUser = $1,
        LockStatusChangeReason = 'Thread was resolved',
        messagecount = $2,
        numberofusersinthread = $3,
        ResolvedTimedate = CURRENT_TIMESTAMP 
        WHERE DiscordThreadLink = $4",
        )
        .bind(resolved_by)
        .bind(message_count)
        .bind(usercount)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Error when updating SQL record after resolving thread: {:?}",
                e
            );
        }
        Ok(())
    }

    pub async fn discord_update_initial_message(
        self,
        author: String,
        contents: String,
        thread_url: String,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET
                    OriginalPoster = $1, 
                    InitialMessage = $2 
                    WHERE DiscordThreadLink = $3",
        )
        .bind(author)
        .bind(contents)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Error when updating SQL record with intitial message: {:?}",
                e
            );
        };
        Ok(())
    }

    pub async fn discord_get_first_response(
        self,
        message_owner: &str,
        thread_url: String,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET
                    FirstResponseUser = $1, 
                    FirstResponseTimedate = CURRENT_TIMESTAMP 
                    WHERE DiscordThreadLink = $2",
        )
        .bind(message_owner)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Error when updating record to show who responded first: {:?}",
                e
            );
        }

        Ok(())
    }

    pub async fn discord_create_issue_record(
        self,
        thread_url: String,
        thread_id: String,
        categories: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query("INSERT INTO issues (
            DiscordThreadId, 
            DiscordThreadLink,
            Categories) 
            VALUES ($1, $2, $3)")
            .bind(thread_id)
            .bind(thread_url)
            .bind(categories)
            .execute(&self.db)
            .await
        {
            error!("Error inserting issue to db while creating new helpthread record: {e:?}");
        }

        Ok(())
    }

    pub async fn discord_set_catsev(
        self,
        severity: i32,
        thread_url: String,
    ) -> Result<(), anyhow::Error> {
        if let Err(e) = sqlx::query(
            "UPDATE issues SET
        SevCat = $1
        WHERE DiscordThreadLink = $2",
        )
        .bind(severity)
        .bind(thread_url)
        .execute(&self.db)
        .await
        {
            error!(
                "Error when updating SQL record after resolving thread: {:?}",
                e
            );
        }
        Ok(())
    }

    pub async fn get_last_four_weeks_stats(self) -> Result<Vec<LastFourWeeksStats>, String> {
        match sqlx::query_as::<_, LastFourWeeksStats>("SELECT
        CONCAT(to_date(concat(DATE_PART('year', date(created)), DATE_PART('week', date(created))), 'iyyyiw'),' - ',to_date(concat('2023', DATE_PART('week', date(created))), 'yyyyww') + 6) AS date_range,
        COUNT(*) as total_issues,
        (SELECT COUNT(*) FROM issues WHERE githubLink IS NOT NULL) as total_elevated_issues,
        (SELECT COUNT(*) FROM issues WHERE resolved = TRUE) as total_resolved_issues,
        (SELECT COUNT(*) FROM issues WHERE messagecount > 6 AND usercount >= 2) as total_one_touch_threads,
        (SELECT COUNT(*) FROM issues WHERE messagecount > 50) as extended_threads,
        CAST((SELECT date_trunc('second', AVG(firstresponsetimedate - created)) FROM issues) as varchar) as average_response_time,
        (SELECT COUNT(ResolverUser) FROM issues WHERE resolved = True group by ResolverUser order by ResolverUser desc limit 1) as best_solver,
        (SELECT COUNT(FirstResponseUser) FROM issues WHERE resolved = True group by FirstResponseUser order by FirstResponseUser desc limit 1) as best_first_responder
        FROM issues
        GROUP BY date_range 
        ORDER BY date_range DESC
        LIMIT 4
        ")
        .fetch_all(&self.db)
        .await {
            Ok(res) => Ok(res),
            Err(e) => Err(format!("Error occurred while getting last 4 weeks stats: {e}"))
        }
    }

    pub async fn get_issues_awaiting_response(self) -> Result<IssuesAwaitingResponse, String> {
        match sqlx::query_as::<_, IssuesAwaitingResponse>("SELECT
        (SELECT COUNT(*) FROM issues WHERE FirstResponseUser IS NULL) as unanswered_threads,
        (SELECT COUNT(*) FROM issues WHERE Resolved = FALSE) as unresolved_issues,
        (SELECT COUNT(*) FROM issues WHERE GithubLink IS NOT NULL and Resolved = FALSE) as unresolved_github_issues
        FROM issues
        ")
        .fetch_one(&self.db)
        .await {
            Ok(res) => Ok(res),
            Err(e) => Err(format!("Error occurred while getting issues awaiting response: {e}"))
        }
    }

    pub async fn get_issues_opened_last_7_days(self) -> Result<Vec<IssuesOpenedLastWeek>, String> {
        match sqlx::query_as::<_, IssuesOpenedLastWeek>(
            "with days as (
        select generate_series(
        date(current_timestamp) - 6,
        date(current_timestamp),
        '1 day'::interval
            ) as day
        )

        select
        CAST(date(days.day) as varchar) as day,
        count(issues.id) as total_issues_per_day
        from days
        left join issues on date(created) = days.day
        group by 1
        order by day desc",
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => Ok(res),
            Err(e) => Err(format!(
                "Error occurred while getting issues opened over last 7 days: {e}"
            )),
        }
    }

    pub async fn get_all_issues(self) -> Result<Vec<Issue>, String> {
        match sqlx::query_as::<_, Issue>(
            "SELECT 
        OriginalPoster as original_poster,
        DiscordThreadLink as discord_thread_link,
        SevCat as severity, 
        FirstResponseUser as first_responder,
        GithubLink as github_link,
        ResolverUser as resolved_by,
        categories,
        CAST(DATE(created) as varchar) as creation_date
        from issues
        ",
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(res) => Ok(res),
            Err(e) => Err(format!(
                "Error occurred while retrieving list of issues: {e}"
            )),
        }
    }
}
