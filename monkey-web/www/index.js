import { Interpreter } from 'monkey-web';

const PROMPT = '>>>';

const interpreter = Interpreter.new();

const inputField = document.getElementById('input');

const stdoutContainer = document.getElementById('stdout');

inputField.addEventListener('keyup', element => {
  if (event.key === 'Enter') {
    const input = element.target.value;
    appendOutput('history-input', `${PROMPT} ${input}`);

    try {
      const output = interpreter.evaluate(input);
      const { stdout, result } = parseOutput(output);
      appendOutput('history-output-ok', stdout);
      appendOutput('history-output-ok', result);
    } catch (error) {
      const { stdout, result } = parseOutput(error);
      appendOutput('history-output-ok', stdout);
      appendOutput('history-output-err', result);
    }

    element.target.value = '';
    stdoutContainer.lastChild.scrollIntoView();
  }
});

const appendOutput = (className, text) => {
  const div = document.createElement('div');
  div.classList.add(className);

  text.split("\n").map(row => {
    const paragraph = document.createElement('p');
    paragraph.classList.add("history-output-line");
    paragraph.textContent = row;
    div.appendChild(paragraph);
  });

  stdoutContainer.appendChild(div);
};

const terminal = document.getElementById('terminal');

terminal.addEventListener('click', _ => {
  inputField.focus();
});

// TODO Better way to pass structs from WASM to JS.
const parseOutput = (output) => {
  console.info(output);
  const splitted = output.split("|");
  if (splitted.length !== 2) {
    throw new Error("Unexpected number of elements after parsing string from WASM")
  }

  return {
    stdout: splitted[0],
    result: splitted[1],
  };
}
