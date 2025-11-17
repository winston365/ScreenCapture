#include <QApplication>

#include "Lang.h"

Lang* lang;

Lang::Lang(QObject *parent) : QObject(parent)
{
	
}

Lang::~Lang()
{

}

QString Lang::get(const QString& name)
{
	return lang->dic[name];
}

void Lang::init()
{
	lang = new Lang(qApp);
	lang->initKo();
}

void Lang::initKo()
{
	dic.insert("saveFile", "파일 저장");

	dic.insert("rect", "사각형");
	dic.insert("ellipse", "원형");
	dic.insert("arrow", "화살표");
	dic.insert("number", "번호");
	dic.insert("line", "선");
	dic.insert("text", "텍스트");
	dic.insert("mosaic", "모자이크");
	dic.insert("eraser", "지우개");
	dic.insert("undo", "실행 취소");
	dic.insert("redo", "다시 실행");
	dic.insert("pin", "고정");
	dic.insert("clipboard", "클립보드에 저장");
	dic.insert("save", "디스크에 저장");
	dic.insert("close", "닫기");

	dic.insert("rectFill", "채워진 사각형");
	dic.insert("strokeCtrl", "선 굵기: ");
	dic.insert("ellipseFill", "채워진 원형");
	dic.insert("arrowFill", "채워진 화살표");
	dic.insert("numberFill", "채워진 번호");
	dic.insert("lineTransparent", "반투명 선");
	dic.insert("bold", "굵게");
	dic.insert("italic", "기울임");
	dic.insert("mosaicFill", "사각형 모자이크");
	dic.insert("eraserFill", "사각형 지우개");


	dic.insert("red", "빨강");
	dic.insert("yellow", "노랑");
	dic.insert("green", "초록");
	dic.insert("cyan", "청록");
	dic.insert("blue", "파랑");
	dic.insert("purple", "보라");
	dic.insert("pink", "분홍");
	dic.insert("black", "검정");

	dic.insert("start", "시작");
	dic.insert("reachBottom", "하단에 도달하여 자동 스크롤이 중지되었습니다");
	dic.insert("tooLong", "이미지가 너무 길어 자동 스크롤이 중지되었습니다");
}
