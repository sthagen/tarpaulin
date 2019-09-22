use cargo::core::compiler::CompileMode;
use clap::arg_enum;
use coveralls_api::CiService;
use std::str::FromStr;
use void::Void;

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
    pub enum RunType {
        Tests,
        Doctests,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum OutputFile {
        Json,
        Toml,
        Stdout,
        Xml,
        Html,
    }
}

impl Default for OutputFile {
    #[inline]
    fn default() -> Self {
        OutputFile::Stdout
    }
}

pub struct Ci(pub CiService);

impl From<RunType> for CompileMode {
    fn from(run: RunType) -> Self {
        match run {
            RunType::Tests => CompileMode::Test,
            RunType::Doctests => CompileMode::Doctest,
        }
    }
}

impl FromStr for Ci {
    /// This can never fail, so the error type is uninhabited.
    type Err = Void;

    #[inline]
    fn from_str(x: &str) -> Result<Ci, Self::Err> {
        match x {
            "circle-ci" => Ok(Ci(CiService::Circle)),
            "codeship" => Ok(Ci(CiService::Codeship)),
            "jenkins" => Ok(Ci(CiService::Jenkins)),
            "semaphore" => Ok(Ci(CiService::Semaphore)),
            "travis-ci" => Ok(Ci(CiService::Travis)),
            "travis-pro" => Ok(Ci(CiService::TravisPro)),
            other => Ok(Ci(CiService::Other(other.to_string()))),
        }
    }
}
