
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'store-cli' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'store-cli'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'store-cli' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('use', 'use', [CompletionResultType]::ParameterValue, 'Use a database table')
            [CompletionResult]::new('tables', 'tables', [CompletionResultType]::ParameterValue, 'List used database tables')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'Count number of stored items')
            [CompletionResult]::new('store', 'store', [CompletionResultType]::ParameterValue, 'Stores file into database')
            [CompletionResult]::new('retrieve', 'retrieve', [CompletionResultType]::ParameterValue, 'Retrieves file from the database and outputs it into the provided file name, or onto stdout if no output file is specified')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'store-cli;use' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
        'store-cli;tables' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
        'store-cli;count' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
        'store-cli;store' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
        'store-cli;retrieve' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
        'store-cli;help' {
            [CompletionResult]::new('-R', 'R', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('--rpc', 'rpc', [CompletionResultType]::ParameterName, 'ZMQ socket for connecting daemon RPC interface')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Set verbosity level')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Set verbosity level')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
