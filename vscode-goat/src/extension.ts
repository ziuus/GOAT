import * as vscode from 'vscode';
import { Client } from '@modelcontextprotocol/sdk/client/index.js';
import { StdioClientTransport } from '@modelcontextprotocol/sdk/client/stdio.js';
import * as path from 'path';
import * as os from 'os';

let mcpClient: Client | undefined;

export async function activate(context: vscode.ExtensionContext) {
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

            const transport = new StdioClientTransport({
                command: goatPath,
                args: ['--mcp-server']
            });

            mcpClient = new Client({
                name: 'vscode-goat',
                version: '0.1.0'
            }, {
                capabilities: {}
            });

            await mcpClient.connect(transport);
            vscode.window.showInformationMessage('GOAT MCP Server connected successfully!');
            
        } catch (error: any) {
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
                const result: any = await mcpClient!.callTool({
                    name: 'goat_repo_status',
                    arguments: {}
                });

                if (result.isError) {
                    vscode.window.showErrorMessage(`GOAT Error: ${result.content[0].text}`);
                } else {
                    const doc = await vscode.workspace.openTextDocument({
                        content: result.content[0].text,
                        language: 'markdown'
                    });
                    await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
                }
            });
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to call goat_repo_status: ${error.message}`);
        }
    });

    context.subscriptions.push(startMcpDisposable, showRepoStatusDisposable);
}

export function deactivate() {
    if (mcpClient) {
        // transport closing would go here
    }
}
