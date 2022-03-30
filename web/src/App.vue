<script setup lang="ts">
import {ref, onMounted} from 'vue'

import * as hawk_run from '@/hawk_layer'

import * as hawk from "@/hawk-wasm/hawk_interface"
import CodeMirror from 'codemirror'
import 'codemirror/lib/codemirror.css'
import 'codemirror/addon/mode/simple'
import './assets/pastel-on-dark.css'

let code = ref("")

let command = ref("")

const editor = ref()

let codeEditor: CodeMirror.EditorFromTextArea;

onMounted(() => {
    CodeMirror.defineSimpleMode("hawk", {
        start: [
        {regex: /"(?:[^\\]|\\.)*?(?:"|$)/, token: "string"},
        // You can match multiple tokens at once. Note that the captured
        // groups must span the whole string in this case
        {regex: /(function)(\s+)([a-z$][\w$]*)/,
        token: ["keyword", "variable-2"]},
        {regex: /(?:if|else|while|for|break|class|super|this|let|const|null|function|return|print|true|false|or|and|not|import|process|finder|find|equation)\b/, token: "keyword"},
        {regex: /true|false|null|undefined/, token: "atom"},
        {regex: /0x[a-f\d]+|[-+]?(?:\.\d+|\d+\.?\d*)(?:e[-+]?\d+)?/i, token: "number"},
        {regex: /\/\/.*/, token: "comment"},
        {regex: /\/(?:[^\\]|\\.)*?\//, token: "comment"},
        {regex: /\/\*/, token: "comment", next: "comment"},
        {regex: /[-+\/*=<>!]+/, token: "operator"},
        {regex: /[A-Z$][A-Z0-9_$]*/, token: "variable"},
        {regex: /\w+\(.*\)/, token: "variable-2"}
        ],

        comment: [
        {regex: /.*?\*\//, token: "comment", next: "start"},
        {regex: /.*/, token: "comment"}
        ],

        meta: {
            lineComment: "//"
        }
    })

    codeEditor = CodeMirror.fromTextArea(editor.value, {
        lineNumbers: true,
        tabSize: 4,
        mode: "hawk",
        theme: "pastel-on-dark",
        indentWithTabs: true
    })

    codeEditor.setSize("50%", "100%")
})


function run_code_repl() {
    if (command.value === "clear" || command.value === "exit") {
        hawk.output.value = []
    } else {
        hawk_run.run_code(command.value, true)
    }
    command.value = ""
}

let current_file = ref(-1)
let new_file_name = ref("")

function select_file(i: number) {
    if (current_file.value !== -1) {
        hawk.files.value[current_file.value].content = codeEditor.getValue()
    }
    current_file.value = i
    codeEditor.setValue(hawk.files.value[current_file.value].content)
    window.localStorage.setItem("hawk_files", JSON.stringify(hawk.files.value))
    console.log(hawk.files.value);
    
}

function new_file() {
    if (new_file_name.value !== "") {
        hawk.files.value.push({name: new_file_name.value, content: ""})
    }
    if (current_file.value == -1) {
        current_file.value = 0
    }
    new_file_name.value  = ""
    window.localStorage.setItem("hawk_files", JSON.stringify(hawk.files.value))
}

function delete_file() {
    hawk.files.value.splice(current_file.value, 1)
    if (current_file.value == hawk.files.value.length) {
        current_file.value--
    }
    codeEditor.setValue(hawk.files.value[current_file.value].content)
    window.localStorage.setItem("hawk_files", JSON.stringify(hawk.files.value))
}

function run_code() {
    hawk.files.value[current_file.value].content = codeEditor.getValue()
    hawk_run.run_code((hawk.files.value.find(el => el.name == "main.hawk") ?? {name: "", content: codeEditor.getValue()}).content, false)
}

</script>

<template>
<div id="main">
    <div id="top-bar">
        <input type="text" placeholder="New File Name" @keydown.enter="new_file" v-model="new_file_name">
        <button class="button-newfile" @click="new_file">New File</button>
        <button class="button-deletefile" @click="delete_file">Delete File</button>
        <div id="button-run-container">
            <button class="button-run" @click="run_code">Run Code</button>
        </div>
    </div>
    <div id="content">
        <div id="files">
            <h4 v-for="(file, i) in hawk.files.value" @click="select_file(i)" :class="{
                file: true,
                highlighted: i == current_file
            }">{{file.name}}</h4>
        </div>

        <textarea ref="editor"></textarea>

        <div id="output">
            <h4 v-for="x in hawk.output.value" :class="{
                print: x.type == hawk.OutputType.Print,
                warn: x.type == hawk.OutputType.Warn,
                err: x.type == hawk.OutputType.Err,
                output: true
            }">{{x.text}}</h4>
                
            <div id="repl-input-inner">
                <input type="text" id="repl-input" v-model="command" @keydown.enter="run_code_repl" placeholder=">> Enter a REPL command">
            </div>
        </div>
    </div>
</div>

</template>

<style>
@import "./assets/base.css";

body {
    background-color: var(--color-background);
    margin: 0;
}

input[type=text] {
    background-color: var(--color-background);
    color: var(--white);
    font-family: jb_mono;
    border: solid 1px;
    outline: none;
    border-radius: 5px;
    padding: 10px;
}

#main {
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
}

#top-bar {
    padding: 10px;
    display: flex;
    flex-direction: row;
    background-color: var(--color-background-mute);
}

#button-run-container {
    flex-grow: 1;
    display: flex;
    justify-content: flex-end;
}

#content {
    display: flex;
    flex-direction: row;
}

#files {
    width: 200px;
    min-height: calc(100vh - 60px);
    background-color: var(--color-background-soft);
}

#output {
    display: flex;
    margin-left: 15px;
    flex-direction: column;
    flex-grow: 1;
    flex-basis: calc(100vh - 60px);
    min-height: calc(100vh - 60px);
    background-color: var(--color-background-soft);
}

#repl-input-inner {
    display: flex;
    margin-top: auto;
    flex-direction: row;
}

#repl-input {
    flex-grow: 1;
    border: none;
    border-radius: 0;
    background-color: var(--color-background-medium);
}


button {
    font-family: jb_mono;
}

.button-run {
    border: solid 1px #37aa79;
    border-radius: 5px;
    height: 40px;
    width: 100px;
    background-color: var(--color-background-soft);
    color: var(--white);
    cursor: pointer;
    transition: all 0.25s;
}

.button-run:hover {
    background-color: #37aa79;
    color: black;
    transition: all 0.25s;
}

.button-newfile {
    border: solid 1px #7276f2;
    border-radius: 5px;
    height: 40px;
    width: 100px;
    background-color: var(--color-background-soft);
    color: var(--white);
    cursor: pointer;
    margin-inline: 10px;
    transition: all 0.25s;
}

.button-newfile:hover {
    background-color: #7276f2;
    color: black;
    transition: all 0.25s;
}

.button-deletefile {
    border: solid 1px #f27272;
    border-radius: 5px;
    height: 40px;
    width: 110px;
    background-color: var(--color-background-soft);
    color: var(--white);
    cursor: pointer;
    margin-inline: 10px;
    transition: all 0.25s;
}

.button-deletefile:hover {
    background-color: #f27272;
    color: black;
    transition: all 0.25s;
}

.CodeMirror {
    font-family: jb_mono;
}

h4 {
    padding-top: 3px;
    margin-top: 0;
    margin-bottom: 0;
    padding-bottom: 3px;
}

.output {
    margin-left: 10px;
    font-size: 0.8rem;
}

.file {
    padding-top: 3px;
    padding-bottom: 4px;
    padding-left: 15px;
    font-size: 0.92rem;
}

.highlighted {
    color: var(--color-background);
    background-color: var(--white);
}

.print {
    color: white;
    font-family: jb_mono;
}
.warn {
    font-family: jb_mono;
    color: orange;
}
.err {
    font-family: jb_mono;
    color: red;
}

textarea {
    resize: none;
    background-color: var(--color-background-soft);
    color: var(--white);
    font-family: jb_mono;
}

textarea:focus {
    outline: none;
}
</style>