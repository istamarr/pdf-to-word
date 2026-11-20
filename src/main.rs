use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf = Document::open(
        "SIK_Single Dashboard Implementasi Collector pada Endpoint PT Pegadaian.pdf",
    )?;
    pdf.save_docx("SIK_Single Dashboard Implementasi Collector pada Endpoint PT Pegadaian.docx")?;
    Ok(())
}
