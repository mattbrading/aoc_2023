use aoc;
use std::collections::HashMap;
fn main() {
    aoc::run(19, |input| {
        let part_1 = PartSystem::from(input).sum_accepted_parts();

        return (Some(part_1), None);
    })
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        // {x=787,m=2655,a=1222,s=2876}
        let p_map: HashMap<&str, &str> = HashMap::from_iter(
            value
                .get(1..value.len() - 1)
                .unwrap()
                .split(",")
                .map(|p| p.split_once("=").unwrap()),
        );

        return Self {
            x: p_map.get("x").unwrap().parse().unwrap(),
            m: p_map.get("m").unwrap().parse().unwrap(),
            a: p_map.get("a").unwrap().parse().unwrap(),
            s: p_map.get("s").unwrap().parse().unwrap(),
        };
    }
}

#[derive(Debug)]
struct PartSystem {
    parts: Vec<Part>,
    workflows: HashMap<String, Vec<String>>,
}

impl From<&str> for PartSystem {
    fn from(value: &str) -> Self {
        let (workflows, parts) = value.split_once("\n\n").unwrap();

        let parts = parts.lines().map(|l| Part::from(l)).collect();

        let workflows = HashMap::from_iter(workflows.lines().map(|l| {
            // px{a<2006:qkq,m>2090:A,rfg}

            let (key, w) = l.split_once("{").unwrap();

            let rules = w
                .replace("}", "")
                .split(',')
                .map(|s| String::from(s))
                .collect();

            (key.to_owned(), rules)
        }));

        return Self { parts, workflows };
    }
}

impl PartSystem {
    fn evalutate_workflow(&self, workflow: &str, part: &Part) -> bool {
        let workflow = self.workflows.get(workflow).expect("Invalid workflow");

        let result = workflow.iter().find_map(|rule| {
            // a<2006:qkq

            let (rule, outcome) = match rule.split_once(":") {
                Some((a, b)) => (Some(a), b),
                None => (None, rule.as_str()),
            };

            let rule = rule.map(|r| {
                let mut r = r.chars();
                let attr = r.next().unwrap();
                let comp = r.next().unwrap();
                let val: u64 = r.collect::<String>().parse().unwrap();

                let p_val = match attr {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Invalid Attr"),
                };

                match comp {
                    '>' => p_val.gt(&val),
                    '<' => p_val.lt(&val),
                    _ => panic!("Invalid comparison"),
                }
            });

            match rule {
                Some(true) => Some(outcome),
                Some(false) => None,
                None => Some(outcome),
            }
        });

        match result {
            Some("A") => true,
            Some("R") => false,
            Some(next) => self.evalutate_workflow(next, part),
            None => panic!("Workflow didn't get a result!"),
        }
    }

    fn is_part_accepted(&self, part: &Part) -> bool {
        self.evalutate_workflow("in", part)
    }

    fn sum_accepted_parts(&self) -> u64 {
        let sum = self
            .parts
            .iter()
            .filter(|p| self.is_part_accepted(p))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum();

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use crate::PartSystem;

    const INPUT: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}\
    ";

    #[test]
    fn test_evaluate_parts() {
        let result = PartSystem::from(INPUT).sum_accepted_parts();

        assert_eq!(result, 19114)
    }
}
