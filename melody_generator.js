// melody_generator.js — JavaScript версия

const fs = require('fs');
const { exec } = require('child_process');

const NOTES = {
    'C4': 261.63, 'C#4': 277.18, 'D4': 293.66, 'D#4': 311.13,
    'E4': 329.63, 'F4': 349.23, 'F#4': 369.99, 'G4': 392.00,
    'G#4': 415.30, 'A4': 440.00, 'A#4': 466.16, 'B4': 493.88,
    'C5': 523.25, 'C#5': 554.37, 'D5': 587.33, 'D#5': 622.25,
    'E5': 659.25, 'F5': 698.46, 'F#5': 739.99, 'G5': 783.99,
    'G#5': 830.61, 'A5': 880.00, 'A#5': 932.33, 'B5': 987.77,
    'C6': 1046.50
};
const DURATIONS = ['1/16', '1/8', '1/4', '1/2', '1'];
const DURATION_VALUES = {'1/16': 0.0625, '1/8': 0.125, '1/4': 0.25, '1/2': 0.5, '1': 1.0};

class MelodyGenerator {
    constructor(numNotes = 8, tempo = 120, start = 'C4', end = 'C6') {
        this.numNotes = numNotes;
        this.tempo = tempo;
        this.notesPool = Object.keys(NOTES).filter(n => n >= start && n <= end);
        if (this.notesPool.length === 0) this.notesPool = Object.keys(NOTES);
        this.melody = [];
    }

    generate() {
        this.melody = [];
        for (let i = 0; i < this.numNotes; i++) {
            const note = this.notesPool[Math.floor(Math.random() * this.notesPool.length)];
            const duration = DURATIONS[Math.floor(Math.random() * DURATIONS.length)];
            this.melody.push([note, duration]);
        }
        return this.melody;
    }

    play() {
        for (const [note, dur] of this.melody) {
            const freq = NOTES[note];
            const durationMs = DURATION_VALUES[dur] * (60 / this.tempo) * 1000;
            const cmd = `beep -f ${Math.round(freq)} -l ${Math.round(durationMs)}`;
            exec(cmd, (err) => { /* ignore */ });
            // Небольшая пауза (не блокируем, т.к. exec запускает асинхронно, но для простоты используем setTimeout)
            // В реальности для последовательности нужна синхронизация, здесь просто ждём.
            // Упрощённо: используем sleep для порядка (блокирующий)
            // В Node.js можно использовать child_process.execSync
            try {
                require('child_process').execSync(`sleep 0.0${Math.round(durationMs/1000*0.1)}`); // грубая пауза
            } catch (e) {}
        }
    }

    save(filename = 'melody.txt') {
        let content = "Note\tFreq (Hz)\tDuration\n";
        for (const [note, dur] of this.melody) {
            content += `${note}\t${NOTES[note].toFixed(2)}\t${dur}\n`;
        }
        fs.writeFileSync(filename, content);
        console.log(`💾 Сохранено: ${filename}`);
    }
}

function main() {
    const args = process.argv.slice(2);
    let numNotes = 8, tempo = 120, range = 'C4-C6', output = 'melody.txt';
    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--notes' || args[i] === '-n') numNotes = parseInt(args[++i]);
        else if (args[i] === '--tempo' || args[i] === '-t') tempo = parseInt(args[++i]);
        else if (args[i] === '--range' || args[i] === '-r') range = args[++i];
        else if (args[i] === '--output' || args[i] === '-o') output = args[++i];
    }
    const parts = range.split('-');
    const start = parts[0] || 'C4';
    const end = parts[1] || 'C6';

    const gen = new MelodyGenerator(numNotes, tempo, start, end);
    gen.generate();
    console.log('🎵 Random Melody Generator (JavaScript)');
    console.log(`Параметры: ${numNotes} нот, диапазон ${start}-${end}, темп ${tempo} BPM\n`);
    console.log('Сгенерированная мелодия:');
    for (const [note, dur] of gen.melody) {
        console.log(`  ${note} (${NOTES[note].toFixed(2)} Гц) ${dur}`);
    }
    console.log('\nВоспроизведение...');
    gen.play();
    gen.save(output);
}

if (require.main === module) main();
