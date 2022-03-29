import {ref} from 'vue'

export enum OutputType {
    Print,
    Warn,
    Err
}

export interface Output {
    text: string
    type: OutputType
}

export interface File {
    name: string
    content: string
}

export let output = ref(<Output[]>[])

export let files = ref([{name: "main.hawk", content: ""}])

let local = window.localStorage.getItem("hawk_files")

if (local !== null) {
    files.value = JSON.parse(local ?? "")
}

export function print(message: string) {
    output.value.push({text: message, type: OutputType.Print})
}

export function warn(message: string) {
    output.value.push({text: message, type: OutputType.Warn})
}

export function error(message: string) {
    output.value.push({text: message, type: OutputType.Err})
}

export function writefile(name: string, content: string) {
    files.value.push({name, content})
}

export function readfile(name: string): string {
    let found = false
    let i = 0
    while (!found && i < files.value.length) {
        if (files.value[i].name == name) {
            return files.value[i].content
        }
        i++
    }
    return ""
}