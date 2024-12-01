namespace AoC.Utils;

public static class Input {
    
    private const string InputPath = "Inputs";

    public static string[] GetInputLines(this string fileName) {
        var file = Directory.GetFiles($"{InputPath}/{fileName}.txt").FirstOrDefault();
        if (file is null) {
            throw new FileNotFoundException();
        }
        
        return File.ReadAllLines(file);
    } 
}


