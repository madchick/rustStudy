use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // 1. 현재 시스템의 가용 논리 코어 수 확인
    let num_threads = rayon::current_num_threads();
    println!("==================================================");
    println!("가용 논리 코어(스레드) 수: {} 개", num_threads);
    println!("작업 관리자 성능 탭을 열고 엔터를 누르세요...");
    println!("==================================================");

    // 사용자가 작업 관리자를 준비할 수 있도록 일시 대기
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!(">> 전체 코어 풀가동 연산 시작! (약 10~15초 소요)");
    let start_time = Instant::now();

    // 2. 대량 데이터(1,000만 개) 생성
    let mut numbers: Vec<f64> = (1..=10_000_000).map(|v| v as f64).collect();

    // 3. Rayon의 par_iter_mut()을 이용해 모든 코어를 동원한 CPU 집약적 연산 수행
    numbers.par_iter_mut().for_each(|n| {
        // CPU를 집중적으로 태우는 무거운 반복 연산 (삼각함수 + 거듭제곱)
        let mut temp = *n;
        for _ in 0..500 {
            temp = (temp.sin() * temp.cos()).abs().sqrt() + 1.0;
        }
        *n = temp;
    });

    let duration = start_time.elapsed();
    println!(">> 연산 완료! 총 소요 시간: {:.2?}", duration);
}