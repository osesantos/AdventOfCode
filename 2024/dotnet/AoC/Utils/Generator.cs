namespace AoC.Utils;

public static class Generator {
    
    public static void GenerateDay(int day) {
        const string templatePath = $"Template/Day.Template.cs";
        var dayName = $"Day{day}";
        var dayPath = $"Days/{dayName}";
        var inputPath = $"Inputs/{dayName}.txt";
        
        if (Directory.Exists(dayPath)) {
            throw new Exception("Day already exists");
        }
        
        Directory.CreateDirectory(dayPath);
        File.Copy(templatePath, $"{dayPath}/{dayName}.cs");
        File.Create(inputPath).Close();
        ReplaceInFile($"{dayPath}/{dayName}.cs", "Template", dayName);
        
        Console.WriteLine($"Day {day} generated");
    }
    
    private static void ReplaceInFile(string filePath, string oldString, string newString) {
        var text = File.ReadAllText(filePath);
        text = text.Replace(oldString, newString);
        File.WriteAllText(filePath, text);
    }
}

