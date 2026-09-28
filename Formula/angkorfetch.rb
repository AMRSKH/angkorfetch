class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/AMRSKH/angkorfetch"
  # No explicit `version` stanza: Homebrew scans it from the vX.Y.Z path segment
  # of the URLs below, and `brew audit --strict` rejects restating it.
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.2/angkorfetch-macos-aarch64.tar.gz"
      sha256 "c19633ebbd56805c21ff9a0fba1765c325a49def5756534b037139d2ad0a521e"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.2/angkorfetch-macos-x86_64.tar.gz"
      sha256 "900502c44a083b413eee20d533e890ee27659bd10e6b65f6db1aefc68967d8fb"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.2/angkorfetch-linux-aarch64.tar.gz"
      sha256 "54b01395d12f43154c88c608fcdaa3ae488f48731b2320778e60818d7df91a20"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.2/angkorfetch-linux-x86_64.tar.gz"
      sha256 "6b866186a4ff1bbc490a3fc7f89e076b8e4acd7bcdaee0db32ba145f7a20f47e"
    end
  end

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{bin}/angkorfetch --version")
  end
end
