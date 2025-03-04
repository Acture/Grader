use crate::check;
use crate::defines::class;
use crate::defines::student::Student;
use dialoguer::{FuzzySelect, Select};
use itertools::Itertools;
use log::info;
use std::collections::HashMap;
use std::process::exit;
use suite::test_suite::{AdditionalStatus, TestResult};

pub fn select_class(classes: &[class::Class]) {
	let class_options = classes
		.iter()
		.enumerate()
		.map(|(_i, class)| class.name.as_str())
		.chain(std::iter::once("退出"))
		.collect::<Vec<_>>();
	let class_selector = Select::new()
		.with_prompt("Please Select a Class")
		.items(&class_options);

	loop {
		let selected_index = class_selector.clone().interact().expect("Select failed");

		match class_options[selected_index] {
			"退出" => return,
			_ => {
				let selected_class = &classes[selected_index];
				info!("所选班级：{}", selected_class.name);
				select_assignment(selected_class);
			}
		}
	}
}

fn select_assignment(selected_class: &class::Class) {
	let assignment_options = selected_class
		.assignments
		.iter()
		.map(|assignment| assignment.name.as_str())
		.chain(vec!["返回", "退出"])
		.collect::<Vec<_>>();
	let assignment_selector = Select::new()
		.with_prompt("Please Select an Assignment")
		.items(&assignment_options);
	loop {
		let selected_index = assignment_selector
			.clone()
			.interact()
			.expect("Select failed");

		match assignment_options[selected_index] {
			"退出" => exit(0),
			"返回" => return,
			_ => {
				let selected_assignment_name = &assignment_options[selected_index];
				info!("所选作业：{}", selected_assignment_name);

				let (submission_map, submission_hash_map) =
					check::check_assignment(selected_class, selected_assignment_name);
				select_submission(&submission_map, &submission_hash_map);
			}
		}
	}
}

fn select_submission(
	submissions: &HashMap<Student, Vec<TestResult>>,
	_hash_map: &HashMap<u64, Vec<Student>>,
) {
	let results_keys = submissions
		.keys()
		.sorted_by(|a, b| a.sis_login_id.cmp(&b.sis_login_id))
		.collect::<Vec<_>>();
	let record_options = results_keys
		.iter()
		.map(|student| -> std::string::String {
			let record = submissions.get(student).expect("Failed to get test result");
			let pass_count = record.iter().filter(|r| r.passed).count();
			let info_count = record.iter().filter(|r| r.infos.is_some()).count();
			let add_info_count = record
				.iter()
				.filter(|r| r.additional_infos.is_some())
				.count();

			let additional_status = if record.is_empty() {
				AdditionalStatus::None
			} else {
				record
					.iter()
					.map(|r| {
						r.additional_status
							.as_ref()
							.unwrap_or(&suite::test_suite::AdditionalStatus::None)
					})
					.fold(AdditionalStatus::Full, |acc, status| {
						if *status == AdditionalStatus::Partial || acc == AdditionalStatus::Partial
						{
							AdditionalStatus::Partial
						} else if *status == AdditionalStatus::None || acc == AdditionalStatus::None
						{
							AdditionalStatus::None
						} else {
							AdditionalStatus::Full
						}
					})
			};

			let hash_collision_status = if _hash_map.iter().filter(|(_, v)| v.len() > 1).any(
                |(_hash, same_hash_students)| {
					same_hash_students
						.iter()
						.any(|same_hash_student| *student == same_hash_student)
				},
			) {
				true
			} else {
				false
			};

			format!(
				"{:<10}\t{:<10}\t{:<4}\t{:>4}\t{:>2}/{:>2}\t{:<5}\t{:>2} infos\t{:>2} add_infos",
				student.name,
				student.sis_login_id,
				match record.is_empty() {
					true => "未提交",
					false => "已提交",
				},
				match hash_collision_status {
					true => "冲突",
					false => "无冲突",
				},
				pass_count,
				record.len(),
				match additional_status {
					AdditionalStatus::None => "未完成附加",
					AdditionalStatus::Partial => "尝试附加",
					AdditionalStatus::Full => "完成附加",
				},
				info_count,
				add_info_count,
			)
		})
		.chain(vec!["返回".to_string(), "退出".to_string()])
		.collect::<Vec<_>>();
	loop {
		let selected_record_index = FuzzySelect::new()
			.with_prompt("Please Select a Record")
			.items(&record_options)
			.interact()
			.expect("Select failed");

		match record_options[selected_record_index].as_str() {
			"返回" => return,
			"退出" => exit(0),
			_ => {
				let selected_record = submissions
					.get(results_keys[selected_record_index])
					.unwrap();
				select_detail(selected_record)
			}
		}
	}
}

fn select_detail(selected_record: &[TestResult]) {
    let mut result_options = Vec::new();
    let mut result_values = Vec::new();

    // 遍历处理每个测试结果
    for (i, result) in selected_record.iter().enumerate() {
        // 处理普通信息
        if let Some(infos) = &result.infos {
            for (k, v) in infos.iter() {
                result_options.push(format!("{}: {}", i, k));
                result_values.push(v.to_string());
            }
        }

        // 处理附加信息
        if let Some(additional_infos) = &result.additional_infos {
            for (k, v) in additional_infos.iter() {
                result_options.push(format!("{}: {}", i, k));
                result_values.push(v.to_string());
            }
        }
    }

    // 添加返回和退出选项
    result_options.push("返回".to_string());
    result_options.push("退出".to_string());


	loop {
		let selected_result_index = FuzzySelect::new()
			.default(0)
			.with_prompt("Please Select a Result")
			.items(&result_options)
			.interact()
			.expect("Select failed");

		let selected_result_keys = result_options[selected_result_index].clone();

		match selected_result_keys.as_str() {
			"返回" => return,
			"退出" => exit(0),
			_ => {
                let selected_result_value = &result_values[selected_result_index];
                info!("{}: {}", selected_result_keys, selected_result_value);
			}
		}
	}
}
