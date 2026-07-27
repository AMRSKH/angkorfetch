class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/AMRSKH/angkorfetch"
  license "MIT"
  version "1.0.1"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.0.1/angkorfetch-macos-aarch64.tar.gz"
      sha256 "b6c46e0e24c9f2fc10349dbe447302ccd9a02b098c287a627353e81199bb1cc9"
    else
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.0.1/angkorfetch-macos-x86_64.tar.gz"
      sha256 "7488a7c0275ebf69577aada2b3ce32028af1476a89fb72cb4c918c9970d84779"
    end
  end

  on_linux do
    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.0.1/angkorfetch-linux-aarch64.tar.gz"
      sha256 "06add6a4aab05eb4ed8061dc487099d2b8f3c203701aa2ab699f18f35399ce93"
    else
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.0.1/angkorfetch-linux-x86_64.tar.gz"
      sha256 "3afd65aa5a030f487fba33995ca56336cd741fa57792a4e71b6c497ebe2941b2"
    end
  end

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{bin}/angkorfetch --version")
  end
end
