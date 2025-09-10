#[cfg(test)]
mod test_models {
    use crate::models::{Status, Action};

    #[test]
    fn test_status_from_string() {
        let status = Status::status_from_str("todo").unwrap();
        assert_eq!(status, Status::Todo);

        let status = Status::status_from_str("in-progress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::status_from_str("done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    #[should_panic]
    fn test_wrong_status_string() {
        let _ = Status::status_from_str("wrong").unwrap();
    }

    #[test]
    fn test_action_from_string() {
        assert_eq!(Action::action_from_str("add").unwrap(), Action::Add);
        assert_eq!(Action::action_from_str("update").unwrap(), Action::Update);
        assert_eq!(Action::action_from_str("delete").unwrap(), Action::Delete);
        assert_eq!(Action::action_from_str("list").unwrap(), Action::List);
        assert_eq!(Action::action_from_str("mark-to-do").unwrap(), Action::Mark(Status::Todo));
        assert_eq!(Action::action_from_str("mark-in-progress").unwrap(), Action::Mark(Status::InProgress));
        assert_eq!(Action::action_from_str("mark-done").unwrap(), Action::Mark(Status::Done));
    }

    #[test]
    #[should_panic]
    fn test_wrong_action_string() {
        let _ = Action::action_from_str("wrong").unwrap();
    }
}

#[cfg(test)]
mod test_database {
    use serial_test::serial;
    use crate::models::Task;
    use crate::database;

    #[test]
    #[serial]
    fn test_create_and_read_task() {
        database::create_task("Unit test task").unwrap();
        let task_list = database::read_task_list(None).unwrap();
        let task = task_list.last().unwrap();

        assert_eq!(task.description, "Unit test task");
    }

    #[test]
    #[serial]
    fn test_update_task() {
        database::create_task("Unit test task").unwrap();
        let task_list = database::read_task_list(None).unwrap();
        let task_count = task_list.len();
        let task = task_list.last().unwrap();
        let updated_task = Task {
            description: "Unit test update task".to_string(),
            ..*task
        };

        assert_eq!(task.description, "Unit test task");
        assert_eq!(updated_task.description, "Unit test update task");

        database::update_task(updated_task).unwrap();

        let task_list = database::read_task_list(None).unwrap();

        assert_eq!(task_list.len(), task_count);
        assert_eq!(task_list.last().unwrap().description, "Unit test update task");
    }

    #[test]
    #[serial]
    fn test_delete_task() {
        database::create_task("Unit test delete task").unwrap();
        let task_list = database::read_task_list(None).unwrap();
        let task_count = task_list.len();

        database::delete_task(task_count-1).unwrap();
        let task_list = database::read_task_list(None).unwrap();
        assert!(task_list.len() < task_count);
    }
}