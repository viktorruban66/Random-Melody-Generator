// melody_generator.go — Go версия

package main

import (
	"flag"
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"runtime"
	"strconv"
	"strings"
	"time"
)

var notes = map[string]float64{
	"C4": 261.63, "C#4": 277.18, "D4": 293.66, "D#4": 311.13,
	"E4": 329.63, "F4": 349.23, "F#4": 369.99, "G4": 392.00,
	"G#4": 415.30, "A4": 440.00, "A#4": 466.16, "B4": 493.88,
	"C5": 523.25, "C#5": 554.37, "D5": 587.33, "D#5": 622.25,
	"E5": 659.25, "F5": 698.46, "F#5": 739.99, "G5": 783.99,
	"G#5": 830.61, "A5": 880.00, "A#5": 932.33, "B5": 987.77,
	"C6": 1046.50,
}

var durations = map[string]float64{
	"1/16": 0.0625, "1/8": 0.125, "1/4": 0.25, "1/2": 0.5, "1": 1.0,
}
var durationKeys = []string{"1/16", "1/8", "1/4", "1/2", "1"}

type MelodyGenerator struct {
	numNotes int
	tempo    int
	notesPool []string
	melody   [][2]string
}

func NewMelodyGenerator(numNotes, tempo int, start, end string) *MelodyGenerator {
	pool := []string{}
	for n := range notes {
		if n >= start && n <= end {
			pool = append(pool, n)
		}
	}
	if len(pool) == 0 {
		for n := range notes {
			pool = append(pool, n)
		}
	}
	return &MelodyGenerator{
		numNotes:  numNotes,
		tempo:     tempo,
		notesPool: pool,
	}
}

func (m *MelodyGenerator) generate() {
	m.melody = nil
	for i := 0; i < m.numNotes; i++ {
		note := m.notesPool[rand.Intn(len(m.notesPool))]
		dur := durationKeys[rand.Intn(len(durationKeys))]
		m.melody = append(m.melody, [2]string{note, dur})
	}
}

func (m *MelodyGenerator) play() {
	for _, pair := range m.melody {
		note := pair[0]
		dur := pair[1]
		freq := notes[note]
		durationMs := durations[dur] * (60.0 / float64(m.tempo)) * 1000.0
		var cmd *exec.Cmd
		switch runtime.GOOS {
		case "windows":
			cmd = exec.Command("powershell", "-Command", fmt.Sprintf("[System.Console]::Beep(%d, %d)", int(freq), int(durationMs)))
		default:
			cmd = exec.Command("beep", "-f", strconv.Itoa(int(freq)), "-l", strconv.Itoa(int(durationMs)))
		}
		cmd.Run()
		time.Sleep(time.Duration(durationMs*0.1) * time.Millisecond)
	}
}

func (m *MelodyGenerator) save(filename string) {
	f, _ := os.Create(filename)
	defer f.Close()
	f.WriteString("Note\tFreq (Hz)\tDuration\n")
	for _, pair := range m.melody {
		f.WriteString(fmt.Sprintf("%s\t%.2f\t%s\n", pair[0], notes[pair[0]], pair[1]))
	}
	fmt.Printf("💾 Сохранено: %s\n", filename)
}

func main() {
	rand.Seed(time.Now().UnixNano())
	numNotes := flag.Int("notes", 8, "Количество нот")
	tempo := flag.Int("tempo", 120, "Темп (BPM)")
	rangeStr := flag.String("range", "C4-C6", "Диапазон")
	output := flag.String("output", "melody.txt", "Файл для сохранения")
	flag.Parse()

	parts := strings.Split(*rangeStr, "-")
	start, end := "C4", "C6"
	if len(parts) == 2 {
		start, end = strings.TrimSpace(parts[0]), strings.TrimSpace(parts[1])
	}

	gen := NewMelodyGenerator(*numNotes, *tempo, start, end)
	gen.generate()
	fmt.Println("🎵 Random Melody Generator (Go)")
	fmt.Printf("Параметры: %d нот, диапазон %s-%s, темп %d BPM\n\n", *numNotes, start, end, *tempo)
	fmt.Println("Сгенерированная мелодия:")
	for _, pair := range gen.melody {
		fmt.Printf("  %s (%.2f Гц) %s\n", pair[0], notes[pair[0]], pair[1])
	}
	fmt.Println("\nВоспроизведение...")
	gen.play()
	gen.save(*output)
}
