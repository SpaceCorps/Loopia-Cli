namespace Loopia.Console.Infrastructure;

public sealed class LoopiaApiException : Exception
{
    public LoopiaApiException(string message) : base(message) { }
}
