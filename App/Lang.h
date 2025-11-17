#pragma once

#include <QObject>
#include <QHash>

class Lang  : public QObject
{
	Q_OBJECT

public:
	Lang(QObject *parent);
	~Lang();
	static QString get(const QString& name);
	static void init();

	void initKo();
private:

	QHash<QString, QString> dic;
};
