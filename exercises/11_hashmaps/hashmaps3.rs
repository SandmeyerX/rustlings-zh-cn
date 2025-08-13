// 已知一场足球比赛的比分列表(每行一个比分)。
// 每行的格式为 "<球队1名称>, <球队2名称>, <球队1进球数>, <球队2进球数>"
// 例如: "英格兰, 法国, 4, 2"(英格兰进了4个球，法国进了2个球)。
//
// 你需要构建一个比分表，其中包含球队名称、球队总进球数以及球队总失球数。

use std::collections::HashMap;

// 一个用于存储球队进球详细信息的结构体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // 球队的名称是键，与之关联的结构体是值。
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 注意: 我们使用 `unwrap` 是因为我们还没有处理错误处理相关内容。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 使用提取出的详细信息来填充比分表。
        // 切记，球队1的进球数将是球队2的失球数。同样，球队2的进球数将是球队1的失球数。
    }

    scores
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
                            France,Italy,3,1
                            Poland,Spain,2,0
                            Germany,England,2,1
                            England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
