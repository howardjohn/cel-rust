// Example demonstrating the closure-based interpreter
// This shows how to use the CompiledExpression for improved performance
// when executing the same expression multiple times with different contexts.

use cel::interpreter::CompiledExpression;
use cel::{Context, Program};

fn main() {
    println!("=== Closure-Based Interpreter Example ===\n");

    // Example 1: Simple arithmetic expression
    println!("Example 1: Simple arithmetic");
    let program = Program::compile("1 + 2 * 3").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    let ctx = Context::default();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: 1 + 2 * 3");
    println!("  Result: {:?}\n", result);

    // Example 2: Using variables
    println!("Example 2: Variables");
    let program = Program::compile("x * 2 + y").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    
    // Execute with different variable values
    let mut ctx = Context::default();
    ctx.add_variable("x", 10i64).unwrap();
    ctx.add_variable("y", 5i64).unwrap();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: x * 2 + y");
    println!("  With x=10, y=5: {:?}", result);
    
    let mut ctx2 = Context::default();
    ctx2.add_variable("x", 20i64).unwrap();
    ctx2.add_variable("y", 3i64).unwrap();
    let result2 = compiled.execute(&ctx2).unwrap();
    println!("  With x=20, y=3: {:?}\n", result2);

    // Example 3: Lists and indexing
    println!("Example 3: Lists");
    let program = Program::compile("[1, 2, 3, 4, 5][2]").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    let ctx = Context::default();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: [1, 2, 3, 4, 5][2]");
    println!("  Result: {:?}\n", result);

    // Example 4: Conditionals
    println!("Example 4: Conditionals");
    let program = Program::compile("x > 10 ? 'large' : 'small'").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    
    let mut ctx = Context::default();
    ctx.add_variable("x", 15i64).unwrap();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: x > 10 ? 'large' : 'small'");
    println!("  With x=15: {:?}", result);
    
    let mut ctx2 = Context::default();
    ctx2.add_variable("x", 5i64).unwrap();
    let result2 = compiled.execute(&ctx2).unwrap();
    println!("  With x=5: {:?}\n", result2);

    // Example 5: Maps
    println!("Example 5: Maps");
    let program = Program::compile("{\"name\": \"Alice\", \"age\": 30}").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    let ctx = Context::default();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: {{\"name\": \"Alice\", \"age\": 30}}");
    println!("  Result: {:?}\n", result);

    // Example 6: Logical operators
    println!("Example 6: Logical operators");
    let program = Program::compile("(x > 5 && x < 15) || y == 0").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    
    let mut ctx = Context::default();
    ctx.add_variable("x", 10i64).unwrap();
    ctx.add_variable("y", 1i64).unwrap();
    let result = compiled.execute(&ctx).unwrap();
    println!("  Expression: (x > 5 && x < 15) || y == 0");
    println!("  With x=10, y=1: {:?}", result);
    
    let mut ctx2 = Context::default();
    ctx2.add_variable("x", 20i64).unwrap();
    ctx2.add_variable("y", 0i64).unwrap();
    let result2 = compiled.execute(&ctx2).unwrap();
    println!("  With x=20, y=0: {:?}\n", result2);

    // Example 7: Performance comparison
    println!("Example 7: Performance comparison");
    println!("  Executing the same expression 100,000 times...");
    
    let program = Program::compile("x * 2 + y * 3 - 10").unwrap();
    let compiled = CompiledExpression::compile(program.expression());
    let mut ctx = Context::default();
    ctx.add_variable("x", 5i64).unwrap();
    ctx.add_variable("y", 7i64).unwrap();
    
    // Traditional interpreter
    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        let _ = program.execute(&ctx).unwrap();
    }
    let traditional_time = start.elapsed();
    
    // Closure-based interpreter
    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        let _ = compiled.execute(&ctx).unwrap();
    }
    let compiled_time = start.elapsed();
    
    println!("  Traditional interpreter: {:?}", traditional_time);
    println!("  Closure-based interpreter: {:?}", compiled_time);
    println!("  Speedup: {:.2}x\n", traditional_time.as_secs_f64() / compiled_time.as_secs_f64());

    println!("=== Examples complete ===");
}
