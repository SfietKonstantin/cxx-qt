#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

void
MyObject::sayHi(const QString& string, int number) const
{
  m_rustObj->sayHi(qStringToRustStr(string), number);
}

void
MyObject::sayBye() const
{
  m_rustObj->sayBye();
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}
