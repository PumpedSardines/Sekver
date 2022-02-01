import * as vscode from "vscode";

export function activate(context: vscode.ExtensionContext) {
  // 👍 formatter implemented using API
  vscode.languages.registerDocumentFormattingEditProvider("sekver", {
    provideDocumentFormattingEdits(
      document: vscode.TextDocument
    ): vscode.TextEdit[] {
      const editorConfig = vscode.workspace.getConfiguration("editor");
      const tabSize = editorConfig.get("tab\Size", 4);

      const edits: vscode.TextEdit[] = [];
      let bracket = 0;

      for (let i = 0; i < document.lineCount; i++) {
        const line = document.lineAt(i);

        const isCloseBracket =
          line.text.match(/^(\s|})+$/) && (line.text.match(/}/g) || []).length;
        if (isCloseBracket) {
          bracket -= isCloseBracket;
        }

        // First set whitespace to current bracket indent
        const deleteRange = new vscode.Range(
          new vscode.Position(i, 0),
          new vscode.Position(i, line.firstNonWhitespaceCharacterIndex)
        );

        edits.push(
          vscode.TextEdit.delete(deleteRange),
          vscode.TextEdit.insert(
            line.range.start,
            String(" ").repeat(tabSize as number).repeat(bracket)
          )
        );

        if (!isCloseBracket) {
          // Then update bracket indent
          const lB = (line.text.match(/{/g) || []).length;
          const rB = (line.text.match(/}/g) || []).length;

          bracket += lB - rB;
        }
      }

      return edits;
    },
  });
}
