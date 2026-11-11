class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/AMRSKH/angkorfetch"
  # No explicit `version` stanza: Homebrew scans it from the vX.Y.Z path segment
  # of the URLs below, and `brew audit --strict` rejects restating it.
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.2.1/angkorfetch-macos-aarch64.tar.gz"
      sha256 "4d09460aaa454709bbd01978b9fbba32c91991ed0e107293d88596ac439ac31d"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.2.1/angkorfetch-macos-x86_64.tar.gz"
      sha256 "f9fc81ef78b7e83dc0f39546ba21982ddd0c5e133c7085a5a6bc9b67ec543b23"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.2.1/angkorfetch-linux-aarch64.tar.gz"
      sha256 "a0dc39947fe224cfe89f580466a4cabaeefd1250acffefcb0ad5f0fab15841d8"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.2.1/angkorfetch-linux-x86_64.tar.gz"
      sha256 "a4e46ab93a9f8252c8c7cfa53d992aa1857023096c3f8cc12d0e3207beabfea2"
    end
  end

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{bin}/angkorfetch --version")
  end
end
