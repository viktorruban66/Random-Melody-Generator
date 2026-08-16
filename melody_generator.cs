// melody_generator.cs — C# версия

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Threading;

class MelodyGenerator
{
    static Dictionary<string, double> NOTES = new Dictionary<string, double>
    {
        {"C4", 261.63}, {"C#4", 277.18}, {"D4", 293.66}, {"D#4", 311.13},
        {"E4", 329.63}, {"F4", 349.23}, {"F#4", 369.99}, {"G4", 392.00},
        {"G#4", 415.30}, {"A4", 440.00}, {"A#4", 466.16}, {"B4", 493.88},
        {"C5", 523.25}, {"C#5", 554.37}, {"D5", 587.33}, {"D#5", 622.25},
        {"E5", 659.25}, {"F5", 698.46}, {"F#5", 739.99}, {"G5", 783.99},
        {"G#5", 830.61}, {"A5", 880.00}, {"A#5", 932.33}, {"B5", 987.77},
        {"C6", 1046.50}
    };
    static string[] DURATIONS = { "1/16", "1/8", "1/4", "1/2", "1" };
    static Dictionary<string, double> DURATION_VALUES = new Dictionary<string, double>
    {
        {"1/16", 0.0625}, {"1/8", 0.125}, {"1/4", 0.25}, {"1/2", 0.5}, {"1", 1.0}
    };

    private int numNotes;
    private int tempo;
    private List<string> notesPool;
    private List<string[]> melody;

    public MelodyGenerator(int numNotes, int tempo, string start, string end)
    {
        this.numNotes = numNotes;
        this.tempo = tempo;
        notesPool = new List<string>();
        foreach (var n in NOTES.Keys)
        {
            if (string.Compare(n, start) >= 0 && string.Compare(n, end) <= 0)
                notesPool.Add(n);
        }
        if (notesPool.Count == 0) notesPool.AddRange(NOTES.Keys);
        melody = new List<string[]>();
    }

    public void Generate()
    {
        melody.Clear();
        Random rand = new Random();
        for (int i = 0; i < numNotes; i++)
        {
            string note = notesPool[rand.Next(notesPool.Count)];
            string dur = DURATIONS[rand.Next(DURATIONS.Length)];
            melody.Add(new string[] { note, dur });
        }
    }

    public void Play()
    {
        foreach (var pair in melody)
        {
            double freq = NOTES[pair[0]];
            double durationMs = DURATION_VALUES[pair[1]] * (60.0 / tempo) * 1000;
            try
            {
                Process.Start("beep", $"-f {(int)freq} -l {(int)durationMs}");
                Thread.Sleep((int)(durationMs * 0.1));
            }
            catch
            {
                Console.Beep((int)freq, (int)durationMs);
            }
        }
    }

    public void Save(string filename)
    {
        using (var writer = new StreamWriter(filename))
        {
            writer.WriteLine("Note\tFreq (Hz)\tDuration");
            foreach (var pair in melody)
                writer.WriteLine($"{pair[0]}\t{NOTES[pair[0]]:F2}\t{pair[1]}");
        }
        Console.WriteLine($"💾 Сохранено: {filename}");
    }

    public static void Main(string[] args)
    {
        int numNotes = 8, tempo = 120;
        string range = "C4-C6", output = "melody.txt";
        for (int i = 0; i < args.Length; i++)
        {
            if (args[i] == "--notes" || args[i] == "-n") numNotes = int.Parse(args[++i]);
            else if (args[i] == "--tempo" || args[i] == "-t") tempo = int.Parse(args[++i]);
            else if (args[i] == "--range" || args[i] == "-r") range = args[++i];
            else if (args[i] == "--output" || args[i] == "-o") output = args[++i];
        }
        var parts = range.Split('-');
        string start = parts[0].Trim();
        string end = parts.Length > 1 ? parts[1].Trim() : "C6";

        var gen = new MelodyGenerator(numNotes, tempo, start, end);
        gen.Generate();
        Console.WriteLine("\u001B[36m🎵 Random Melody Generator (C#)\u001B[0m");
        Console.WriteLine($"Параметры: {numNotes} нот, диапазон {start}-{end}, темп {tempo} BPM\n");
        Console.WriteLine("Сгенерированная мелодия:");
        foreach (var pair in gen.melody)
            Console.WriteLine($"  {pair[0]} ({NOTES[pair[0]]:F2} Гц) {pair[1]}");
        Console.WriteLine("\nВоспроизведение...");
        gen.Play();
        gen.Save(output);
    }
}
