use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut results = HashMap::new();

    for score in match_results.split("\n") {
        if let Some((m1,m2))= get_result(score){
            results.entry(m1.team_name.clone()).or_insert(MatchInfo::new(m1.team_name.as_str()));
            results.get_mut(&m1.team_name).unwrap().win += m1.win;
            results.get_mut(&m1.team_name).unwrap().tied += m1.tied;
            results.get_mut(&m1.team_name).unwrap().lost += m1.lost;

            results.entry(m2.team_name.clone()).or_insert(MatchInfo::new(m2.team_name.as_str()));
            results.get_mut(&m2.team_name).unwrap().win += m2.win;
            results.get_mut(&m2.team_name).unwrap().tied += m2.tied;
            results.get_mut(&m2.team_name).unwrap().lost += m2.lost;
        }
    }

    let mut ve: Vec<&MatchInfo> = results.values().collect();
    ve.sort_by(|&a, &b| {
        if a.win*3+a.tied != b.win*3+b.tied {
            return (b.win*3+b.tied).cmp(&(a.win*3+a.tied))
        }
        a.team_name.cmp(&b.team_name)
    });

    let mut rt = "Team                           | MP |  W |  D |  L |  P\n".to_string();
    rt.push_str(&ve.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
    rt
}

pub struct MatchInfo {
    team_name: String,
    win: usize,
    tied: usize,
    lost: usize,
}

impl MatchInfo {
    pub fn new(name:&str) -> Self {
        MatchInfo {
            team_name: name.to_string(),
            win: 0,
            tied: 0,
            lost: 0,
        }
    }
    pub fn construct(name:&str,w:usize,t:usize,l:usize) -> Self {
        MatchInfo {
            team_name: name.to_string(),
            win: w,
            tied: t,
            lost: l,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{:30} |  {} |  {} |  {} |  {} |  {}",
                self.team_name, self.tied + self.win + self.lost
                , self.win, self.tied, self.lost, self.win * 3 + self.tied)
    }
}

fn get_result(res: &str) -> Option<(MatchInfo, MatchInfo)> {
    let ams  = res.split(';').collect::<Vec<_>>();
    if ams.len() == 3 {
        let (m1,m2,ans) = (ams[0],ams[1],ams[2]);
        return match ans {
            "win"=>Some((MatchInfo::construct(m1,1,0,0),MatchInfo::construct(m2,0,0,1))),
            "draw"=>Some((MatchInfo::construct(m1,0,1,0),MatchInfo::construct(m2,0,1,0))),
            "loss"=>Some((MatchInfo::construct(m1,0,0,1),MatchInfo::construct(m2,1,0,0))),
            _=>None,
        };
    }
    None
}