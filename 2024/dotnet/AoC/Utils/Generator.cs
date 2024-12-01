namespace AoC.Utils;

public static class Generator {
    
    public static void GenerateDay(int day) {
        const string templatePath = $"AoC/Template/Day.Template.cs";
        var dayName = $"Day{day}";
        var dayPath = $"AoC/Days/{dayName}";
        var inputPath = $"AoC/Inputs";
        
        if (!Directory.Exists(dayPath)) {
            Directory.CreateDirectory(dayPath);
        }
        
        if (!Directory.Exists(inputPath)) {
            Directory.CreateDirectory(inputPath);
        }
        
        File.Copy(templatePath, $"{dayPath}/{dayName}.cs");
        ReplaceInFile($"{dayPath}/{dayName}.cs", "Template", dayName);
        
        File.Create($"{inputPath}/{dayName}.txt").Close();
        
        Console.WriteLine($"Day {day} generated");
    }
    
    public static void GenerateTests(int day) {
        const string templatePath = $"AoC.UnitTests/TemplateUnitTests/DayTemplate.Tests.cs";
        var dayName = $"Day{day}";
        var dayPath = $"AoC.UnitTests/{dayName}Tests";
        
        if (Directory.Exists(dayPath)) {
            throw new Exception("Day already exists");
        }
        
        Directory.CreateDirectory(dayPath);
        File.Copy(templatePath, $"{dayPath}/{dayName}.Tests.cs");
        ReplaceInFile($"{dayPath}/{dayName}.Tests.cs", "TemplateUnitTests", $"{dayName}Tests");
        ReplaceInFile($"{dayPath}/{dayName}.Tests.cs", "Template", dayName);
        
        Console.WriteLine($"Tests for Day {day} generated");
    }
    
    private static void ReplaceInFile(string filePath, string oldString, string newString) {
        var text = File.ReadAllText(filePath);
        text = text.Replace(oldString, newString);
        File.WriteAllText(filePath, text);
    }
}

