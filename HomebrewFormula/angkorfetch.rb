class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/AMRSKH/angkorfetch"
  license "MIT"
  version "1.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.0/angkorfetch-macos-aarch64.tar.gz"
      sha256 "6ee5d137c52b6ff0d57d7a493d2fb21d31686b0297cdf864da25aa95a1d2c8a5"
    else
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.0/angkorfetch-macos-x86_64.tar.gz"
      sha256 "958d1cf8114930a1ad9ea5136b505adb41c40f27909081cfca09aa6f6c55bb57"
    end
  end

  on_linux do
    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.0/angkorfetch-linux-aarch64.tar.gz"
      sha256 "6b76e2293a9fdcf5f5164509d0aa3ad36eecbb3d64a9f93883e416aae7564f72"
    else
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.0/angkorfetch-linux-x86_64.tar.gz"
      sha256 "803cdc46884ef1c55cbf17795b07dd541cd1f739ab775ceeed5d1b1b72ad5051"
    end
  end

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{bin}/angkorfetch --version")
  end
end
