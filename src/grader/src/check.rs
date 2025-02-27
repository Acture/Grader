use crate::defines::class;
use crate::defines::student::Student;
use lab::TestSuiteType;
use log::warn;
use runner;
use std::any::Any;
use std::collections::HashMap;
use suite::test_suite;
use suite::test_suite::TestResult;

pub fn check_assignment(
	selected_class: &class::Class,
	selected_assignment_name: &str,
) -> HashMap<Student, Vec<TestResult>> {
	let student_assignments =
		selected_class.get_student_assignments(selected_assignment_name.to_string());

	let select_assignment_type = TestSuiteType::from_endwith(selected_assignment_name);

	let solution_map = lab::get_solution_map();
	let test_suite = solution_map
		.get(&select_assignment_type)
		.expect("未找到测试套件");

	student_assignments
		.iter()
		.map(|(student, files)| {
			match files.len() {
				0 => {
					warn!(
						"{} - {}: 未找到作业文件",
						student.name, student.sis_login_id
					);
					return (student.clone(), vec![]);
				}
				1 => (),
				_ => {
					warn!(
						"{} - {}: 作业文件数量不为1",
						student.name, student.sis_login_id
					);
					warn!("{:?}", files)
				}
			}

			let file = files.first().expect("Failed to get file");
			let result = test_suite.run(file);
			let answer = test_suite.get_answer();

			(student.clone(), test_suite.judge(&result, &answer))
		})
		.collect::<HashMap<Student, Vec<TestResult>>>()
}
