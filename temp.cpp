// Código generado automáticamente por lce (LCE -> C++/Qt).
// No editar manualmente: cualquier cambio se perderá.
#include <QApplication>
#include <QMainWindow>
#include <QWidget>
#include <QVBoxLayout>
#include <QPushButton>
#include <QLabel>
#include <QLineEdit>
#include <QMessageBox>
#include <QObject>
#include <QString>

static QMainWindow* crear_ventana(const QString& titulo, int ancho, int alto) {
    auto* ventana = new QMainWindow();
    ventana->setWindowTitle(titulo);
    ventana->resize(ancho, alto);
    return ventana;
}

static QPushButton* crear_boton(const QString& texto) {
    return new QPushButton(texto);
}

static QLabel* crear_etiqueta(const QString& texto) {
    return new QLabel(texto);
}

static QLineEdit* crear_caja_texto(const QString& marcador) {
    auto* caja = new QLineEdit();
    caja->setPlaceholderText(marcador);
    return caja;
}

static QString texto(QLineEdit* caja) {
    return caja->text();
}

static void establecer_texto(QLineEdit* caja, const QString& valor) {
    caja->setText(valor);
}

static void mostrar_mensaje(const QString& titulo, const QString& texto) {
    QMessageBox::information(nullptr, titulo, texto);
}

static void agregar(QMainWindow* ventana, QWidget* widget) {
    auto* central = ventana->centralWidget();
    QVBoxLayout* layout = nullptr;
    if (central == nullptr) {
        central = new QWidget();
        layout = new QVBoxLayout(central);
        ventana->setCentralWidget(central);
    } else {
        layout = qobject_cast<QVBoxLayout*>(central->layout());
    }
    layout->addWidget(widget);
}

static void mostrar(QWidget* widget) {
    widget->show();
}


int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    QMainWindow* v = crear_ventana(QString("Saludo"), 420, 220);
    QLineEdit* nombre = crear_caja_texto(QString("Escribe tu nombre"));
    QPushButton* b = crear_boton(QString("Saludar"));
    QObject::connect(b, &QPushButton::clicked, [&]() {
        QString n = texto(nombre);
        mostrar_mensaje(QString("Saludo"), (QString("Hola, ") + n));
    });
    agregar(v, nombre);
    agregar(v, b);
    mostrar(v);

    return app.exec();
}
