#[cfg(test)]
mod tests {
    use crate::{mealy::Mealy, FSM};

    #[test]
    fn mealy() {
        let mut fsm = Mealy::new("Start".to_string());
        fsm.add_state("Go".into());
        fsm.add_state("Completed".into());
        fsm.add_state("Failed".into());
        fsm.try_add_transition("Start".into(), 0, "Go".into(), "A")
            .unwrap();
        fsm.try_add_transition("Go".into(), 1, "Completed".into(), "B")
            .unwrap();
        fsm.try_add_transition("Go".into(), 2, "Failed".into(), "C")
            .unwrap();
        assert_eq!("Start", fsm.state());
        assert_eq!(None, fsm.next(3));
        assert_eq!("Start", fsm.state());
        assert_eq!(Some("A"), fsm.next(0));
        assert_eq!("Go", fsm.state());
        assert_eq!(Some("B"), fsm.next(1));
        assert_eq!("Completed", fsm.state());
    }

    #[test]
    fn mealy_serialize_yaml() {
        let mut fsm = Mealy::new("Start".to_string());
        fsm.add_state("Go".into());
        fsm.add_state("Completed".into());
        fsm.add_state("Failed".into());
        fsm.try_add_transition("Start".into(), 0, "Go".into(), "A")
            .unwrap();
        fsm.try_add_transition("Go".into(), 1, "Completed".into(), "B")
            .unwrap();
        fsm.try_add_transition("Go".into(), 2, "Failed".into(), "C")
            .unwrap();

        let mealy_str = serde_yaml::to_string(&fsm).unwrap();
        let f: Mealy<_, _, _> = serde_yaml::from_str(&mealy_str).unwrap();
        assert_eq!(fsm, f);
    }
}
