# frozen_string_literal: true

Gem::Specification.new do |s|
  s.name        = 'hyeonbot'
  s.version     = '1.0.1'

  s.files       = ['exe/hyeonbot']
  s.authors     = ['Hyeon Kim']
  s.summary     = 'Personal IRC bot of Hyeon Kim'

  s.description = <<~'EOF'
    Personal IRC bot of Hyeon Kim.
    Daum Dictionary API written in ruby.
  EOF
  s.email       = 'simnalamburt@gmail.com'
  s.homepage    = 'https://github.com/simnalamburt/hyeonbot'
  s.licenses    = ['Apache-2.0', 'MIT']
  s.metadata = {
    'bug_tracker_uri'   => 'https://github.com/simnalamburt/hyeonbot/issues',
    'source_code_uri'   => 'https://github.com/simnalamburt/hyeonbot',
  }

  s.add_runtime_dependency 'mcinch', '~> 2.4'
  s.add_runtime_dependency 'daumdic', '~> 1.0'
  s.add_runtime_dependency 'sqlite3', '~> 1.4'

  s.bindir = 'exe'
  s.executables = 'hyeonbot'
end
