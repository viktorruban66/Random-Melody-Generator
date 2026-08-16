# melody_generator.rb — Ruby версия

NOTES = {
  'C4' => 261.63, 'C#4' => 277.18, 'D4' => 293.66, 'D#4' => 311.13,
  'E4' => 329.63, 'F4' => 349.23, 'F#4' => 369.99, 'G4' => 392.00,
  'G#4' => 415.30, 'A4' => 440.00, 'A#4' => 466.16, 'B4' => 493.88,
  'C5' => 523.25, 'C#5' => 554.37, 'D5' => 587.33, 'D#5' => 622.25,
  'E5' => 659.25, 'F5' => 698.46, 'F#5' => 739.99, 'G5' => 783.99,
  'G#5' => 830.61, 'A5' => 880.00, 'A#5' => 932.33, 'B5' => 987.77,
  'C6' => 1046.50
}
DURATIONS = ['1/16', '1/8', '1/4', '1/2', '1']
DURATION_VALUES = {'1/16' => 0.0625, '1/8' => 0.125, '1/4' => 0.25, '1/2' => 0.5, '1' => 1.0}

class MelodyGenerator
  def initialize(num_notes, tempo, start, end)
    @num_notes = num_notes
    @tempo = tempo
    @notes_pool = NOTES.keys.select { |n| n >= start && n <= end }
    @notes_pool = NOTES.keys if @notes_pool.empty?
    @melody = []
  end

  def generate
    @melody = []
    @num_notes.times do
      note = @notes_pool.sample
      dur = DURATIONS.sample
      @melody << [note, dur]
    end
  end

  def play
    @melody.each do |note, dur|
      freq = NOTES[note]
      duration_ms = DURATION_VALUES[dur] * (60.0 / @tempo) * 1000
      system("beep -f #{freq.to_i} -l #{duration_ms.to_i}")
      sleep(duration_ms * 0.1 / 1000)
    end
  end

  def save(filename)
    File.open(filename, 'w') do |f|
      f.puts "Note\tFreq (Hz)\tDuration"
      @melody.each do |note, dur|
        f.puts "#{note}\t#{NOTES[note].round(2)}\t#{dur}"
      end
    end
    puts "💾 Сохранено: #{filename}"
  end
end

def main
  num_notes = 8
  tempo = 120
  range = 'C4-C6'
  output = 'melody.txt'

  args = ARGV
  i = 0
  while i < args.size
    case args[i]
    when '--notes', '-n' then num_notes = args[i+1].to_i; i += 2
    when '--tempo', '-t' then tempo = args[i+1].to_i; i += 2
    when '--range', '-r' then range = args[i+1]; i += 2
    when '--output', '-o' then output = args[i+1]; i += 2
    else i += 1
    end
  end
  parts = range.split('-')
  start = parts[0] || 'C4'
  end_ = parts[1] || 'C6'

  gen = MelodyGenerator.new(num_notes, tempo, start, end_)
  gen.generate
  puts "\e[36m🎵 Random Melody Generator (Ruby)\e[0m"
  puts "Параметры: #{num_notes} нот, диапазон #{start}-#{end_}, темп #{tempo} BPM\n"
  puts "Сгенерированная мелодия:"
  gen.melody.each { |note, dur| puts "  #{note} (#{NOTES[note].round(2)} Гц) #{dur}" }
  puts "\nВоспроизведение..."
  gen.play
  gen.save(output)
end

main if __FILE__ == $0
