"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const index_js_1 = require("@modelcontextprotocol/sdk/client/index.js");
const stdio_js_1 = require("@modelcontextprotocol/sdk/client/stdio.js");
const path = __importStar(require("path"));
let mcpClient;
async function activate(context) {
    console.log('GOAT VS Code extension is now active!');
    // Command to start MCP server
    let startMcpDisposable = vscode.commands.registerCommand('goat.startMcp', async () => {
        try {
            vscode.window.showInformationMessage('Starting GOAT MCP Server...');
            // Assume the `goat` binary is built and available at workspace root for now
            // Or use cargo run if in dev mode
            let workspaceFolders = vscode.workspace.workspaceFolders;
            if (!workspaceFolders) {
                vscode.window.showErrorMessage('No workspace folder open');
                return;
            }
            const goatPath = path.join(workspaceFolders[0].uri.fsPath, 'target', 'debug', 'goat');
            const transport = new stdio_js_1.StdioClientTransport({
                command: goatPath,
                args: ['--mcp-server']
            });
            mcpClient = new index_js_1.Client({
                name: 'vscode-goat',
                version: '0.1.0'
            }, {
                capabilities: {}
            });
            await mcpClient.connect(transport);
            vscode.window.showInformationMessage('GOAT MCP Server connected successfully!');
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to start GOAT MCP Server: ${error.message}`);
        }
    });
    // Command to show repo status
    let showRepoStatusDisposable = vscode.commands.registerCommand('goat.repoStatus', async () => {
        if (!mcpClient) {
            vscode.window.showErrorMessage('GOAT MCP Server is not running. Start it first.');
            return;
        }
        try {
            vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: "Fetching GOAT Repo Status",
                cancellable: false
            }, async () => {
                const result = await mcpClient.callTool({
                    name: 'goat_repo_status',
                    arguments: {}
                });
                if (result.isError) {
                    vscode.window.showErrorMessage(`GOAT Error: ${result.content[0].text}`);
                }
                else {
                    const doc = await vscode.workspace.openTextDocument({
                        content: result.content[0].text,
                        language: 'markdown'
                    });
                    await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
                }
            });
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to call goat_repo_status: ${error.message}`);
        }
    });
    context.subscriptions.push(startMcpDisposable, showRepoStatusDisposable);
}
function deactivate() {
    if (mcpClient) {
        // transport closing would go here
    }
}
//# sourceMappingURL=extension.js.map