// MelodyGenerator.java — Java версия

import java.io.*;
import java.util.*;

public class MelodyGenerator {
    private static final Map<String, Double> NOTES = new LinkedHashMap<>();
    static {
        NOTES.put("C4", 261.63); NOTES.put("C#4", 277.18);
        NOTES.put("D4", 293.66); NOTES.put("D#4", 311.13);
        NOTES.put("E4", 329.63); NOTES.put("F4", 349.23);
        NOTES.put("F#4", 369.99); NOTES.put("G4", 392.00);
        NOTES.put("G#4", 415.30); NOTES.put("A4", 440.00);
        NOTES.put("A#4", 466.16); NOTES.put("B4", 493.88);
        NOTES.put("C5", 523.25); NOTES.put("C#5", 554.37);
        NOTES.put("D5", 587.33); NOTES.put("D#5", 622.25);
        NOTES.put("E5", 659.25); NOTES.put("F5", 698.46);
        NOTES.put("F#5", 739.99); NOTES.put("G5", 783.99);
        NOTES.put("G#5", 830.61); NOTES.put("A5", 880.00);
        NOTES.put("A#5", 932.33); NOTES.put("B5", 987.77);
        NOTES.put("C6", 1046.50);
    }
    private static final String[] DURATIONS = {"1/16", "1/8", "1/4", "1/2", "1"};
    private static final Map<String, Double> DURATION_VALUES = new HashMap<>();
    static {
        DURATION_VALUES.put("1/16", 0.0625);
        DURATION_VALUES.put("1/8", 0.125);
        DURATION_VALUES.put("1/4", 0.25);
        DURATION_VALUES.put("1/2", 0.5);
        DURATION_VALUES.put("1", 1.0);
    }

    private int numNotes;
    private int tempo;
    private List<String> notesPool;
    private List<String[]> melody;

    public MelodyGenerator(int numNotes, int tempo, String start, String end) {
        this.numNotes = numNotes;
        this.tempo = tempo;
        this.notesPool = new ArrayList<>();
        for (String n : NOTES.keySet()) {
            if (n.compareTo(start) >= 0 && n.compareTo(end) <= 0) {
                notesPool.add(n);
            }
        }
        if (notesPool.isEmpty()) notesPool.addAll(NOTES.keySet());
        this.melody = new ArrayList<>();
    }

    public void generate() {
        melody.clear();
        Random rand = new Random();
        for (int i = 0; i < numNotes; i++) {
            String note = notesPool.get(rand.nextInt(notesPool.size()));
            String dur = DURATIONS[rand.nextInt(DURATIONS.length)];
            melody.add(new String[]{note, dur});
        }
    }

    public void play() {
        for (String[] pair : melody) {
            double freq = NOTES.get(pair[0]);
            double durationMs = DURATION_VALUES.get(pair[1]) * (60.0 / tempo) * 1000;
            try {
                // Используем beep через Runtime (только Linux/macOS)
                Runtime.getRuntime().exec(new String[]{"beep", "-f", String.valueOf((int)freq), "-l", String.valueOf((int)durationMs)});
                Thread.sleep((long)(durationMs * 0.1));
            } catch (Exception e) {
                // ignore
            }
        }
    }

    public void save(String filename) throws IOException {
        try (PrintWriter pw = new PrintWriter(new FileWriter(filename))) {
            pw.println("Note\tFreq (Hz)\tDuration");
            for (String[] pair : melody) {
                pw.printf("%s\t%.2f\t%s\n", pair[0], NOTES.get(pair[0]), pair[1]);
            }
        }
        System.out.println("💾 Сохранено: " + filename);
    }

    public static void main(String[] args) throws Exception {
        int numNotes = 8, tempo = 120;
        String range = "C4-C6", output = "melody.txt";
        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("--notes") || args[i].equals("-n")) numNotes = Integer.parseInt(args[++i]);
            else if (args[i].equals("--tempo") || args[i].equals("-t")) tempo = Integer.parseInt(args[++i]);
            else if (args[i].equals("--range") || args[i].equals("-r")) range = args[++i];
            else if (args[i].equals("--output") || args[i].equals("-o")) output = args[++i];
        }
        String[] parts = range.split("-");
        String start = parts[0].trim();
        String end = (parts.length > 1) ? parts[1].trim() : "C6";

        MelodyGenerator gen = new MelodyGenerator(numNotes, tempo, start, end);
        gen.generate();
        System.out.println("\u001B[36m🎵 Random Melody Generator (Java)\u001B[0m");
        System.out.printf("Параметры: %d нот, диапазон %s-%s, темп %d BPM\n\n", numNotes, start, end, tempo);
        System.out.println("Сгенерированная мелодия:");
        for (String[] pair : gen.melody) {
            System.out.printf("  %s (%.2f Гц) %s\n", pair[0], NOTES.get(pair[0]), pair[1]);
        }
        System.out.println("\nВоспроизведение...");
        gen.play();
        gen.save(output);
    }
}
