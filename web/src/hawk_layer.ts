import init, {Runner} from '@/hawk-wasm/pkg/hawk_wasm'
import * as hawk_interface from '@/hawk-wasm/hawk_interface'

await init()

let code_runner = Runner.new()

let repl_runner = Runner.new()

export function run_code(code: string, in_repl: boolean) {
    if (in_repl) {
        try {
            repl_runner.run(code, true)
        } catch (error: any) {
            hawk_interface.error(`Uncaught runtime error: ${error.message}`)
            repl_runner = Runner.new()
        }
    } else {
        try {
            code_runner.run(code, false)
        } catch (error: any) {
            hawk_interface.error(`Uncaught runtime error: ${error.message}`)
            code_runner = Runner.new()
        }
        code_runner.clear()
    }
}