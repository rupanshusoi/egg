use crate::*;

fn add_reachable<L: Language, A: Analysis<L>>(
    id: Id,
    egraph: &EGraph<L, A>,
    whitelist: &mut HashSet<Id>,
) {
    if whitelist.contains(&id) {
        return;
    }
    whitelist.insert(id);
    for node in egraph[id].nodes.iter() {
        for child in node.children().iter() {
            add_reachable(*child, egraph, whitelist);
        }
    }
}

fn filter<L: Language, A: Analysis<L>>(egraph: &EGraph<L, A>) -> EGraph<L, A>
where
    A: Clone + Default,
    A::Data: Clone,
{
    let ids = egraph
        .classes
        .iter()
        .filter(|(_, ec)| ec.version == egraph.version)
        .map(|(id, _)| *id)
        .collect::<HashSet<_>>();

    let mut whitelist = HashSet::default();
    for id in ids {
        if !whitelist.contains(&id) {
            add_reachable(id, egraph, &mut whitelist);
        }
    }

    // Do something dumb for now...
    let mut new = EGraph::default();
    new.unionfind = egraph.unionfind.clone();
    egraph.classes.iter().for_each(|(id, ec)| {
        if whitelist.contains(id) {
            dbg!();
            new.classes.insert(*id, ec.clone());
        }
    });
    // new.classes.retain(|id, _| whitelist.contains(id));
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_simple() {
        let mut eg: EGraph<SymbolLang, ()> = EGraph::default();
        eg.add_expr(&"(f x)".parse().unwrap());
        eg.version += 1;
        eg.add_expr(&"(g x)".parse().unwrap());
        let new = filter(&eg);
        println!("{:?}", new.dump());
    }
}
