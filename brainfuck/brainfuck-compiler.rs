/** Compila una cadena de caracteres de brainfuck introducida por la stdio. La compila a C, y es
 * responsabilidad del usuario compilarlo definitivamente.
 *
 * # Instrucciones
 * Instala `rustc` y compila el programa. Después puedes ejecutarlo de la siguiente forma:
 * ```
 * cat <file>.bf | ./brainfuck-compiler > <file>.c
 * gcc <file>.c -o <file>
 * ```
 * # Licencia
 * [Unlicense](http://unlicense.org/)
 */

use std::io::stdin;
use std::io::Read;

fn main() {
    let mut code = String::from(r#"#include <stdio.h>

int main()
{
int buffer[1024] = {0};
int *ptr = buffer;

"#);

    for c in stdin().bytes() {
        match c.unwrap() as char {
            '+' => code.push_str("(*ptr)++;\n"),
            '-' => code.push_str("(*ptr)--;\n"),
            '>' => code.push_str("ptr++;\n"),
            '<' => code.push_str("ptr--;\n"),
            '[' => code.push_str("while(*ptr) {\n"),
            ']' => code.push_str("}\n"),
            '.' => code.push_str("putchar(*ptr);\n"),
            ',' => code.push_str("*ptr = getchar();\n"),
            _ => (),
        }
    }

    code.push_str(r#"
return 0;
}
"#);
    println!("{}", code);
}
