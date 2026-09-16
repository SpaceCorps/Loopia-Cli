using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;

namespace Loopia.Console.Infrastructure.Accounts;

/// <summary>
/// Encrypts account passwords at rest so the store never holds plain text.
/// On Windows this is DPAPI bound to the current user; elsewhere it is AES-GCM
/// with a key kept in a 0600 key file next to the store.
/// </summary>
public static class SecretProtector
{
    private static readonly byte[] Entropy = "Loopia.Console/accounts/v1"u8.ToArray();

    public static string Describe() => OperatingSystem.IsWindows()
        ? "DPAPI (current Windows user)"
        : "AES-GCM (local key file)";

    public static string Protect(string plainText)
    {
        var bytes = Encoding.UTF8.GetBytes(plainText);
        return Convert.ToBase64String(OperatingSystem.IsWindows()
            ? ProtectedData.Protect(bytes, Entropy, DataProtectionScope.CurrentUser)
            : AesProtect(bytes));
    }

    public static string Unprotect(string cipherText)
    {
        byte[] bytes;
        try
        {
            bytes = Convert.FromBase64String(cipherText);
        }
        catch (FormatException)
        {
            throw new InvalidOperationException("The stored password is corrupt. Recreate the account with 'loopia account create'.");
        }

        try
        {
            var plain = OperatingSystem.IsWindows()
                ? ProtectedData.Unprotect(bytes, Entropy, DataProtectionScope.CurrentUser)
                : AesUnprotect(bytes);
            return Encoding.UTF8.GetString(plain);
        }
        catch (CryptographicException)
        {
            throw new InvalidOperationException(
                "The stored password could not be decrypted. It was encrypted for a different user or machine; " +
                "recreate the account with 'loopia account create'.");
        }
    }

    private const int NonceSize = 12;
    private const int TagSize = 16;

    private static byte[] AesProtect(byte[] plain)
    {
        var result = new byte[NonceSize + plain.Length + TagSize];
        var nonce = result.AsSpan(0, NonceSize);
        RandomNumberGenerator.Fill(nonce);

        using var aes = new AesGcm(LoadOrCreateKey(), TagSize);
        aes.Encrypt(nonce, plain, result.AsSpan(NonceSize, plain.Length), result.AsSpan(NonceSize + plain.Length, TagSize));
        return result;
    }

    private static byte[] AesUnprotect(byte[] cipher)
    {
        if (cipher.Length < NonceSize + TagSize)
            throw new CryptographicException("Cipher text is too short.");

        var plain = new byte[cipher.Length - NonceSize - TagSize];
        using var aes = new AesGcm(LoadOrCreateKey(), TagSize);
        aes.Decrypt(
            cipher.AsSpan(0, NonceSize),
            cipher.AsSpan(NonceSize, plain.Length),
            cipher.AsSpan(NonceSize + plain.Length, TagSize),
            plain);
        return plain;
    }

    private static byte[] LoadOrCreateKey()
    {
        var path = AccountStore.KeyFilePath;
        if (File.Exists(path))
            return Convert.FromBase64String(File.ReadAllText(path).Trim());

        var key = RandomNumberGenerator.GetBytes(32);
        Directory.CreateDirectory(Path.GetDirectoryName(path)!);
        File.WriteAllText(path, Convert.ToBase64String(key));
        RestrictToOwner(path);
        return key;
    }

    /// <summary>Makes a file readable and writable by its owner only. No-op on Windows, where ACLs already inherit.</summary>
    public static void RestrictToOwner(string path)
    {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            return;
        File.SetUnixFileMode(path, UnixFileMode.UserRead | UnixFileMode.UserWrite);
    }
}
