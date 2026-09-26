class RlLang < Formula
  desc "Programming language with first-class VM and C transpiler"
  homepage "https://github.com/rl-lang/rl-lang"
  url "https://github.com/rl-lang/rl-lang/archive/refs/tags/v2.3.0.tar.gz"
  sha256 "PLACEHOLDER" # update with actual sha256
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    man1.install "man/rl.1"
    info.install "man/rl.info"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rl --version")
  end
end
