<?php
// melody_generator.php — PHP версия

$NOTES = [
    'C4' => 261.63, 'C#4' => 277.18, 'D4' => 293.66, 'D#4' => 311.13,
    'E4' => 329.63, 'F4' => 349.23, 'F#4' => 369.99, 'G4' => 392.00,
    'G#4' => 415.30, 'A4' => 440.00, 'A#4' => 466.16, 'B4' => 493.88,
    'C5' => 523.25, 'C#5' => 554.37, 'D5' => 587.33, 'D#5' => 622.25,
    'E5' => 659.25, 'F5' => 698.46, 'F#5' => 739.99, 'G5' => 783.99,
    'G#5' => 830.61, 'A5' => 880.00, 'A#5' => 932.33, 'B5' => 987.77,
    'C6' => 1046.50
];
$DURATIONS = ['1/16', '1/8', '1/4', '1/2', '1'];
$DURATION_VALUES = ['1/16' => 0.0625, '1/8' => 0.125, '1/4' => 0.25, '1/2' => 0.5, '1' => 1.0];

class MelodyGenerator {
    private $numNotes;
    private $tempo;
    private $notesPool;
    private $melody = [];

    public function __construct($numNotes, $tempo, $start, $end) {
        $this->numNotes = $numNotes;
        $this->tempo = $tempo;
        $this->notesPool = array_filter(array_keys($GLOBALS['NOTES']), function($n) use ($start, $end) {
            return $n >= $start && $n <= $end;
        });
        if (empty($this->notesPool)) $this->notesPool = array_keys($GLOBALS['NOTES']);
    }

    public function generate() {
        $this->melody = [];
        for ($i = 0; $i < $this->numNotes; $i++) {
            $note = $this->notesPool[array_rand($this->notesPool)];
            $dur = $GLOBALS['DURATIONS'][array_rand($GLOBALS['DURATIONS'])];
            $this->melody[] = [$note, $dur];
        }
    }

    public function play() {
        foreach ($this->melody as $pair) {
            list($note, $dur) = $pair;
            $freq = $GLOBALS['NOTES'][$note];
            $durationMs = $GLOBALS['DURATION_VALUES'][$dur] * (60 / $this->tempo) * 1000;
            exec("beep -f " . round($freq) . " -l " . round($durationMs));
            usleep($durationMs * 100);
        }
    }

    public function save($filename) {
        $fp = fopen($filename, 'w');
        fwrite($fp, "Note\tFreq (Hz)\tDuration\n");
        foreach ($this->melody as $pair) {
            fwrite($fp, $pair[0] . "\t" . number_format($GLOBALS['NOTES'][$pair[0]], 2) . "\t" . $pair[1] . "\n");
        }
        fclose($fp);
        echo "💾 Сохранено: $filename\n";
    }
}

function main($argv) {
    $numNotes = 8;
    $tempo = 120;
    $range = 'C4-C6';
    $output = 'melody.txt';

    for ($i = 1; $i < count($argv); $i++) {
        if ($argv[$i] == '--notes' || $argv[$i] == '-n') $numNotes = (int)$argv[++$i];
        elseif ($argv[$i] == '--tempo' || $argv[$i] == '-t') $tempo = (int)$argv[++$i];
        elseif ($argv[$i] == '--range' || $argv[$i] == '-r') $range = $argv[++$i];
        elseif ($argv[$i] == '--output' || $argv[$i] == '-o') $output = $argv[++$i];
    }
    $parts = explode('-', $range);
    $start = trim($parts[0] ?? 'C4');
    $end = trim($parts[1] ?? 'C6');

    $gen = new MelodyGenerator($numNotes, $tempo, $start, $end);
    $gen->generate();
    echo "\033[36m🎵 Random Melody Generator (PHP)\033[0m\n";
    echo "Параметры: $numNotes нот, диапазон $start-$end, темп $tempo BPM\n\n";
    echo "Сгенерированная мелодия:\n";
    foreach ($gen->melody as $pair) {
        echo "  {$pair[0]} (" . number_format($GLOBALS['NOTES'][$pair[0]], 2) . " Гц) {$pair[1]}\n";
    }
    echo "\nВоспроизведение...\n";
    $gen->play();
    $gen->save($output);
}

$argc = $_SERVER['argc'] ?? 0;
$argv = $_SERVER['argv'] ?? [];
main($argv);
?>
